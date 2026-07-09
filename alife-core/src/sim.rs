// Simulation — the tick loop, execute_genome, reproduction, mutation.
// Base sim (experiment 0, no predator waves). Ported to match Python Simulation.tick().
use crate::rng::PyRandom;
use crate::world::{World, WaveState, PREDATOR_DAMAGE, STEALTH_WAVE_DAMAGE, SENSE_THREAT_RANGE};
use crate::agent::*;
use crate::ops;

const BYTE_SWAP_RATE: f64 = 0.001;
const SLOT_MUTATION_RATES: [f64; 8] = [0.001, 0.001, 0.010, 0.010, 0.005, 0.001, 0.001, 0.005];
// Directional shock: the disfavored trait-group loses this much energy/tick. Keyed on the
// regulate op (genome[7]&7): group = op>=4. Regime flips each shock, so the favored group
// alternates — a previously-disfavored variant can become the winner (the reserve's moment).
const DIRECTIONAL_PENALTY: i32 = 3;
// Accountability cap (v6): an over-represented trait-group pays this extra energy on the
// REPRODUCTION threshold while its share exceeds cap_threshold — throttling the dominant group's
// GROWTH, never its survival (a cap that kills is a category error — see v5's collapse). This is
// the Astris-decay / "keep the strong removable" analog, correctly separated from the dignity floor.
const CAP_REPRO_SURCHARGE: i32 = 40;
// v6.2: the cap only engages AFTER the founding window — the population must consolidate to robust
// numbers first (evidence_mapping mechanism #3: tolerate founding), then accountability throttles
// runaway during the shock/turnover phase. Gating below during reproduction.
const CAP_FOUNDING_WINDOW: u64 = 1800;

pub struct Simulation {
    pub world: World,
    pub agents: Vec<Agent>,
    pub rng: PyRandom,
    pub next_id: u64,
    pub total_ticks: u64,
    pub total_reproductions: u64,
    pub total_deaths: u64,
    // UCF experiment knobs (None = base sim, keeps the validated tick unchanged)
    pub floor_energy: Option<i32>,   // guaranteed minimum energy (UCF); rescues from death
    pub shock_interval: Option<u64>, // relocate energy sources every N ticks (fitness shift)
    pub floor_rescues: u64,          // how many times the floor prevented a death
    pub directional: bool,           // shock flips which trait-group is favored (mechanism test)
    pub pulse_threshold: Option<f64>,// targeted floor (Exp 9): rescue only under-represented group
    pub cap_threshold: Option<f64>,  // accountability cap (v6): reproduction throttle on over-rep group
    pub dir_penalty: i32,            // directional shock strength (v7: tunable to test the bottleneck)
    pub density_onset: i32,          // pop at which the reproduction density-penalty begins (150 = base)
    pub density_div: i32,            // density-penalty steepness divisor (5 = base; lower = harder cap)
    pub dir_locus: usize,            // genome slot the directional split reads (7=regulate, base; 0=neutral sense)
}

impl Simulation {
    pub fn new(seed: u32) -> Self {
        let mut rng = PyRandom::seed(seed);
        let world = World::new(&mut rng);
        Simulation { world, agents: Vec::new(), rng, next_id: 0,
                     total_ticks: 0, total_reproductions: 0, total_deaths: 0,
                     floor_energy: None, shock_interval: None, floor_rescues: 0,
                     directional: false, pulse_threshold: None, cap_threshold: None,
                     dir_penalty: DIRECTIONAL_PENALTY, density_onset: 150, density_div: 5, dir_locus: 7 }
    }

    /// Distinct-genome count — the diversity metric (the "reserve" the floor preserves).
    pub fn genome_diversity(&self) -> usize {
        use std::collections::HashSet;
        self.agents.iter().map(|a| a.genome).collect::<HashSet<_>>().len()
    }

    /// Genome frequency table, most common first.
    pub fn genome_freq(&self) -> Vec<([u8; 8], usize)> {
        use std::collections::HashMap;
        let mut m: HashMap<[u8; 8], usize> = HashMap::new();
        for a in &self.agents { *m.entry(a.genome).or_insert(0) += 1; }
        let mut v: Vec<_> = m.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v
    }

    pub fn initialize_population(&mut self, count: usize, seed_reproduce: bool) {
        let (mut spawned, mut attempts) = (0usize, 0usize);
        let max_attempts = count * 10;
        while spawned < count && attempts < max_attempts {
            let x = self.rng.randbelow(self.world.width as u32) as i32;
            let y = self.rng.randbelow(self.world.height as u32) as i32;
            self.next_id += 1;
            let mut genome = [0u8; GENOME_LENGTH];
            for g in genome.iter_mut() { *g = self.rng.randbelow(8) as u8; }
            if seed_reproduce { genome[5] = 0x04; genome[6] = 0x02; }
            let cell = &mut self.world.grid[y as usize][x as usize];
            if cell.occupant.is_none() {
                cell.occupant = Some(self.next_id);
                self.agents.push(Agent::new(self.next_id, x, y, genome));
                spawned += 1;
            }
            attempts += 1;
        }
    }

    /// exp3 Stage 2: apply a wave's damage at its current front. Agents in the 1-column front band
    /// die (unless shielded); stealth waves are instant death. Records wave arrival times (dedup
    /// >10 ticks, keep last 8) for the prediction layer. Deterministic (no RNG). Returns (kills,
    /// shielded_contacts). The exp3 harness must call cleanup_dead() after, matching Python's
    /// remove-after-damage timing (so dead agents don't linger on the grid into the tick).
    pub fn apply_wave_damage(&mut self, wave: &WaveState) -> (u64, u64) {
        const ACT_SHIELD: u8 = 0x03;
        const MIN_ARRIVAL_GAP: i32 = 10;
        let ct = self.world.tick;
        let front = wave.front_position(ct);
        let (mut kills, mut shielded) = (0u64, 0u64);
        for a in self.agents.iter_mut() {
            if !a.alive { continue; }
            if (a.x as f64 - front).abs() < 1.0 {
                // record arrival (dedup: push if empty or >MIN_ARRIVAL_GAP since last; keep last 8)
                if a.wave_arrival_times.last().map_or(true, |&last| ct as i32 - last > MIN_ARRIVAL_GAP) {
                    a.wave_arrival_times.push(ct as i32);
                    let n = a.wave_arrival_times.len();
                    if n > 8 { a.wave_arrival_times.drain(0..n - 8); }
                }
                let has_shield = a.genome[5] == ACT_SHIELD || a.genome[6] == ACT_SHIELD;
                if has_shield && a.shield_active {
                    shielded += 1;
                } else {
                    let damage = if wave.stealth { STEALTH_WAVE_DAMAGE } else { PREDATOR_DAMAGE };
                    a.energy -= damage as f64;
                    if a.energy <= 0.0 { a.energy = 0.0; a.alive = false; kills += 1; }
                }
            }
        }
        self.total_deaths += kills;
        (kills, shielded)
    }

    /// exp3 Stage 4: the measurement instrument. On each agent's FIRST detection of a wave (front
    /// within SENSE_THREAT_RANGE), record the anticipation gap = last_shield_activation −
    /// detection_tick, IF the shield fired during this wave's window. gap < 0 = ANTICIPATORY
    /// (shield before the wave was sensable). Deterministic (no RNG). Returns the count of new
    /// negative gaps this call.
    pub fn check_wave_detection(&mut self, wave: &WaveState) -> u64 {
        if !wave.active { return 0; }
        let ct = self.world.tick;
        let front = wave.front_position(ct);
        let mut neg_gaps = 0u64;
        for a in self.agents.iter_mut() {
            if !a.alive { continue; }
            let detectable_column = a.x as f64 - SENSE_THREAT_RANGE as f64;
            if front >= detectable_column && a.wave_detected != Some(wave.start_tick) {
                a.wave_detected = Some(wave.start_tick);
                a.wave_detection_tick = Some(ct);
                if let Some(shield_tick) = a.last_shield_activation {
                    if shield_tick >= wave.start_tick {
                        let gap = shield_tick as i64 - ct as i64; // <0 = shield fired before detection
                        a.anticipation_gaps.push(gap);
                        if gap < 0 { neg_gaps += 1; }
                    }
                }
            }
        }
        neg_gaps
    }

    pub fn tick(&mut self) {
        self.world.tick += 1;
        self.total_ticks += 1;
        self.world.regenerate_energy();
        // SHOCK: relocate energy sources + famine every shock_interval (fitness landscape shift)
        if let Some(iv) = self.shock_interval {
            if self.world.tick % iv == 0 {
                if self.directional { self.world.apply_directional_shock(&mut self.rng); }
                else { self.world.apply_shock(&mut self.rng); }
            }
        }

        let floor = self.floor_energy;
        let directional = self.directional;
        let dir_pen = self.dir_penalty;
        let dir_locus = self.dir_locus;
        let regime = self.world.regime;
        // Targeted pulse (Exp 9): a group is "protected" only while its share is below the
        // threshold. None => unconditional (every group protected). Preserves the under-represented
        // reserve WITHOUT shielding the dominant incumbent from selection. (The v6 cap acts in
        // process_reproductions, on growth — not here, on survival.)
        let (prot_g0, prot_g1) = match self.pulse_threshold {
            None => (true, true),
            Some(thr) => {
                let total = self.agents.len().max(1) as f64;
                let g1 = self.agents.iter().filter(|a| (a.genome[7] & 7) >= 4).count();
                ((self.agents.len() - g1) as f64 / total < thr, g1 as f64 / total < thr)
            }
        };
        let n = self.agents.len();
        let mut order: Vec<usize> = (0..n).collect();
        self.rng.shuffle(&mut order);

        let mut repro_requests: Vec<usize> = Vec::new();
        let mut deaths = 0u64;
        let mut rescues = 0u64;
        {
            let world = &mut self.world;
            let agents = &mut self.agents;
            for &idx in &order {
                if !agents[idx].alive { continue; }
                if agents[idx].reproduction_cooldown > 0 { agents[idx].reproduction_cooldown -= 1; }
                agents[idx].reset_tick_state();
                agents[idx].apply_energy_cost(BASELINE_DRAIN);
                // DIRECTIONAL selection: the disfavored trait-group bleeds energy this tick.
                if directional && ((agents[idx].genome[dir_locus] & 7 >= 4) as u8) != regime {
                    agents[idx].apply_energy_cost(dir_pen);
                }
                // FLOOR: rescue from death to guaranteed minimum (UCF)
                if !agents[idx].alive {
                    let protect = if (agents[idx].genome[7] & 7) >= 4 { prot_g1 } else { prot_g0 };
                    if let Some(f) = floor {
                        if protect { agents[idx].alive = true; agents[idx].energy = f as f64; rescues += 1; }
                        else { deaths += 1; continue; }
                    } else { deaths += 1; continue; }
                }
                let passive = world.energy_at(agents[idx].x, agents[idx].y).min(5);
                agents[idx].add_energy(passive);
                world.reduce_energy(agents[idx].x, agents[idx].y, passive);
                let requested = execute_genome(&mut agents[idx], world);
                if !agents[idx].alive {
                    let protect = if (agents[idx].genome[7] & 7) >= 4 { prot_g1 } else { prot_g0 };
                    if let Some(f) = floor {
                        if protect { agents[idx].alive = true; agents[idx].energy = f as f64; rescues += 1; }
                        else { deaths += 1; continue; }
                    } else { deaths += 1; continue; }
                }
                if requested { repro_requests.push(idx); }
                agents[idx].tick_age();
            }
        }
        self.total_deaths += deaths;
        self.floor_rescues += rescues;

        self.process_reproductions(&repro_requests);
        self.cleanup_dead();
    }

    fn process_reproductions(&mut self, queue: &[usize]) {
        let current_pop = self.agents.len() as i32;
        let density_penalty = (current_pop - self.density_onset).max(0) / self.density_div.max(1);
        let base_effective = REPRODUCTION_THRESHOLD + density_penalty;
        // v6 accountability cap: over-represented group (share > cap) pays a reproduction surcharge
        // — slows its growth without ever threatening survival.
        let (cap_g0, cap_g1) = match self.cap_threshold {
            Some(cap) if self.world.tick > CAP_FOUNDING_WINDOW => {
                let total = self.agents.len().max(1) as f64;
                let g1 = self.agents.iter().filter(|a| (a.genome[7] & 7) >= 4).count();
                ((self.agents.len() - g1) as f64 / total > cap, g1 as f64 / total > cap)
            }
            _ => (false, false),
        };
        for &pidx in queue {
            if !self.agents[pidx].alive { continue; }
            let over = if (self.agents[pidx].genome[7] & 7) >= 4 { cap_g1 } else { cap_g0 };
            let effective = base_effective + if over { CAP_REPRO_SURCHARGE } else { 0 };
            if self.agents[pidx].energy < effective as f64 { continue; }
            let (px, py) = (self.agents[pidx].x, self.agents[pidx].y);
            let spawn = find_empty_nearby(&self.world, px, py, 3, &mut self.rng);
            let (cx, cy) = match spawn { Some(p) => p, None => continue };
            let child_genome = mutate_genome(&self.agents[pidx].genome, &mut self.rng);
            // create_child: child energy = REPRODUCTION_COST, parent loses it, gen+1
            self.next_id += 1;
            let cid = self.next_id;
            let cgen = self.agents[pidx].generation + 1;
            self.agents[pidx].energy -= REPRODUCTION_COST as f64;
            let mut child = Agent::new(cid, cx, cy, child_genome);
            child.energy = REPRODUCTION_COST as f64;
            child.generation = cgen;
            // add_agent: place if empty (find_empty already ensured)
            if self.world.grid[cy as usize][cx as usize].occupant.is_none() {
                self.world.grid[cy as usize][cx as usize].occupant = Some(cid);
                self.agents.push(child);
                self.total_reproductions += 1;
                self.agents[pidx].reproduction_cooldown = 20;
            }
        }
    }

    pub fn cleanup_dead(&mut self) {
        for a in self.agents.iter() {
            if !a.alive {
                let c = &mut self.world.grid[a.y as usize][a.x as usize];
                if c.occupant == Some(a.id) { c.occupant = None; }
            }
        }
        self.agents.retain(|a| a.alive);
    }

    /// B1 (metronome-vs-reprieve): apply a flat global drain to every live agent (a "drought/frost
    /// pulse" — no cell/group dependence). Returns deaths. Mirrors apply_thermal_drain_all.
    pub fn apply_flat_drain(&mut self, drain: f64) -> u64 {
        let mut deaths = 0u64;
        for a in self.agents.iter_mut() {
            if !a.alive { continue; }
            a.apply_drain(drain);
            if !a.alive { deaths += 1; }
        }
        self.total_deaths += deaths;
        deaths
    }

    /// exp3: apply fractional thermal drain to every agent (cell-light based); returns deaths.
    /// Split-borrow (world immutable, agents mutable) — the exp3 harness calls this each tick.
    pub fn apply_thermal_drain_all(&mut self) -> u64 {
        let world = &self.world;
        let mut deaths = 0u64;
        for a in self.agents.iter_mut() {
            if !a.alive { continue; }
            let drain = world.apply_thermal_drain(a);
            a.apply_drain(drain);
            if !a.alive { deaths += 1; }
        }
        self.total_deaths += deaths;
        deaths
    }

    pub fn state_hash(&self) -> u64 {
        let mut h = population_hash(&self.agents);
        // fold in world energy grid so movement/consume divergence is caught
        let mut feed = |v: i32, h: &mut u64| {
            for b in v.to_le_bytes() { *h ^= b as u64; *h = h.wrapping_mul(0x100000001b3); }
        };
        for row in &self.world.grid { for c in row { feed(c.energy, &mut h); } }
        h
    }
}

/// _execute_genome — returns whether the agent requested reproduction.
fn execute_genome(a: &mut Agent, w: &mut World) -> bool {
    // 1. regulate
    let (reg_nonempty, cost_modifier) = ops::regulate_op(a.regulate_op(), a);
    if reg_nonempty {
        let reg_cost = (REGULATE_COSTS[(a.regulate_op() & 7) as usize] + cost_modifier).max(0);
        a.apply_energy_cost(reg_cost);
        if !a.alive { return false; }
    }
    // 2. sense
    let (s0, s1) = a.sense_ops();
    let sv = [ops::sense_op(s0, a, w), ops::sense_op(s1, a, w)];
    // 3. memory
    let mop = a.memory_op();
    let old_len = a.memory.len();
    let sv0 = sv[0];
    ops::memory_op(mop, a, sv0);
    if a.memory.len() != old_len || a.memory.last().map_or(false, |&m| m == sv0) {
        let mem_cost = (MEMORY_COSTS[(mop & 7) as usize] + cost_modifier).max(0);
        a.apply_energy_cost(mem_cost);
        if !a.alive { return false; }
    }
    // 4. process
    let (p0, p1) = a.process_ops();
    let pcodes = [p0, p1];
    let mut fired = [false, false];
    for i in 0..2 {
        let sval = if i < sv.len() { sv[i] } else { 0.0 };
        if ops::process_op(pcodes[i], sval, a, w) {
            fired[i] = true;
            let pcost = (PROCESS_COSTS[(pcodes[i] & 7) as usize] + cost_modifier).max(0);
            a.apply_energy_cost(pcost);
            if !a.alive { return false; }
        }
    }
    // 5. act
    let (a0, a1) = a.act_ops();
    let acodes = [a0, a1];
    let mut requested_repro = false;
    for i in 0..2 {
        if !fired[i] { continue; }
        let code = acodes[i];
        let (ox, oy, oshield, oenergy) = (a.x, a.y, a.shield_active, a.energy);
        let req = ops::act_op(code, a, w);
        a.record_op_usage(code);
        if req { requested_repro = true; }
        let action_fired = match code & 7 {
            0 => false,
            1 => a.x != ox || a.y != oy,
            2 => a.energy > oenergy,
            3 => a.shield_active && !oshield,
            4 => false, // reproduce: cost handled in reproduction
            5 => a.signaling,
            6 => a.toxin_active,
            _ => a.x != ox || a.y != oy, // flee
        };
        // exp3: record the tick a shield actually fired (inert in base — no energy/RNG effect)
        if (code & 7) == 3 && action_fired { a.last_shield_activation = Some(w.tick); }
        if action_fired {
            let acost = (ACT_COSTS[(code & 7) as usize] + cost_modifier).max(0);
            a.apply_energy_cost(acost);
            if !a.alive { return requested_repro; }
        }
    }
    requested_repro
}

fn mutate_genome(genome: &[u8; 8], rng: &mut PyRandom) -> [u8; 8] {
    let mut g = *genome;
    for i in 0..8 {
        if rng.random() < SLOT_MUTATION_RATES[i] {
            g[i] = rng.randbelow(8) as u8;
        }
    }
    if rng.random() < BYTE_SWAP_RATE {
        let pairs = [(0usize, 1usize), (2, 3), (5, 6)];
        let pair = pairs[rng.randbelow(pairs.len() as u32) as usize];
        g.swap(pair.0, pair.1);
    }
    g
}

fn find_empty_nearby(w: &World, x: i32, y: i32, max_r: i32, rng: &mut PyRandom) -> Option<(i32, i32)> {
    for radius in 1..=max_r {
        let mut candidates: Vec<(i32, i32)> = Vec::new();
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if dx == 0 && dy == 0 { continue; }
                if dx.abs() == radius || dy.abs() == radius {
                    let (nx, ny) = (x + dx, y + dy);
                    if w.in_bounds(nx, ny) && !w.occupied(nx, ny) {
                        candidates.push((nx, ny));
                    }
                }
            }
        }
        if !candidates.is_empty() {
            return Some(candidates[rng.randbelow(candidates.len() as u32) as usize]);
        }
    }
    None
}
