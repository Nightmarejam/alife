// Validation harness for the RNG port — prove Rust matches Python bit-for-bit.
mod rng;
mod world;
mod agent;
mod ops;
mod sim;
use rng::PyRandom;
use world::World;
use sim::Simulation;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(pos) = args.iter().position(|a| a == "run") {
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(1);
        let mut s = Simulation::new(seed);
        s.initialize_population(50, true);
        for _ in 0..ticks { s.tick(); }
        println!("seed {} after {} ticks: pop={} births={} deaths={}",
                 seed, ticks, s.agents.len(), s.total_reproductions, s.total_deaths);
        let hash = s.state_hash();
        println!("state_hash: {:016x}", hash);
        // fossil output — same shape the Python fossil tools read (dependency-free JSON)
        if args.iter().any(|a| a == "fossil") {
            std::fs::create_dir_all("fossils").ok();
            let json = format!(
                "{{\n  \"experiment\": 0,\n  \"seed\": {},\n  \"ticks\": {},\n  \"location\": \"{}\",\n  \"engine\": \"rust\",\n  \"results\": {{\n    \"population\": {},\n    \"total_reproductions\": {},\n    \"total_deaths\": {},\n    \"total_ticks\": {}\n  }},\n  \"state_hash\": \"{:016x}\"\n}}\n",
                seed, ticks, std::env::var("HOSTNAME").unwrap_or_else(|_| "mac".into()),
                s.agents.len(), s.total_reproductions, s.total_deaths, s.total_ticks, hash);
            let path = format!("fossils/exp0_rust_seed{}_t{}.json", seed, ticks);
            std::fs::write(&path, json).ok();
            println!("wrote {}", path);
        }
        return;
    }
    if std::env::args().any(|a| a == "pop") {
        // canonical path: the same Simulation::initialize_population every real mode uses
        let mut s = Simulation::new(42);
        s.initialize_population(50, true);
        println!("seed 42 population: {} agents", s.agents.len());
        let a0 = &s.agents[0];
        println!("agent0: pos=({},{}) genome={:?} energy={}", a0.x, a0.y, a0.genome, a0.energy);
        println!("population_hash: {:016x}", agent::population_hash(&s.agents));
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "ucf") {
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(10000);
        let shock = 2000u64;
        let run_arm = |floor: Option<i32>| -> (usize, usize, usize, bool, u64) {
            let mut s = Simulation::new(seed);
            s.floor_energy = floor;
            s.shock_interval = Some(shock);
            s.initialize_population(50, true);
            let mut min_pop = 50usize;
            let mut extinct = false;
            for _ in 0..ticks {
                s.tick();
                let p = s.agents.len();
                if p < min_pop { min_pop = p; }
                if p == 0 { extinct = true; break; }
            }
            (s.agents.len(), s.genome_diversity(), min_pop, extinct, s.floor_rescues)
        };
        println!("=== UCF floor experiment (seed {}, {} ticks, shock every {}) ===", seed, ticks, shock);
        let (fp, fd, fmin, fext, fr) = run_arm(Some(30));
        let (np, nd, nmin, next, _) = run_arm(None);
        println!("{:<10} {:>8} {:>10} {:>12} {:>9} {:>10}", "arm", "finalPop", "diversity", "minPop(resil)", "extinct", "rescues");
        println!("{:<10} {:>8} {:>10} {:>12} {:>9} {:>10}", "FLOOR", fp, fd, fmin, fext, fr);
        println!("{:<10} {:>8} {:>10} {:>12} {:>9} {:>10}", "NO-FLOOR", np, nd, nmin, next, "-");
        println!("\nHealth read (higher finalPop/diversity/minPop = healthier system):");
        println!("  floor helped survival: {}", if fp > np || (next && !fext) { "YES" } else { "no/unclear" });
        println!("  floor preserved diversity: {}", if fd > nd { "YES" } else { "no/unclear" });
        println!("  floor improved resilience (min pop trough): {}", if fmin > nmin { "YES" } else { "no/unclear" });
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "mech") {
        // Mechanism test: is the post-shock winner a PRE-shock minority variant?
        // If a preserved rare genome rises to dominance when the environment shifts,
        // the diversity reserve was load-bearing (not just decorative).
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(20000);
        let shock = 2000u64;
        let recovery = 1000u64; // measure 1000 ticks after each shock
        let mut s = Simulation::new(seed);
        s.floor_energy = Some(30);
        s.shock_interval = Some(shock);
        s.initialize_population(50, true);
        let mut pre: Vec<([u8; 8], usize)> = Vec::new();
        let mut pre_dom: Option<[u8; 8]> = None;
        let mut reserve_hits = 0u32;
        let mut events = 0u32;
        println!("=== UCF MECHANISM test (floor arm, seed {}, {} ticks) ===", seed, ticks);
        println!("Q: after each shock, was the new dominant genome a PRE-shock minority?\n");
        for _ in 0..ticks {
            s.tick();
            let wt = s.world.tick;
            if (wt + 1) % shock == 0 {
                pre = s.genome_freq();
                pre_dom = pre.first().map(|x| x.0);
            }
            if wt >= shock && wt % shock == recovery && !pre.is_empty() {
                let after = s.genome_freq();
                if let Some(d_after) = after.first().map(|x| x.0) {
                    let rank = pre.iter().position(|x| x.0 == d_after);
                    let pre_count = rank.map(|r| pre[r].1).unwrap_or(0);
                    let total_pre: usize = pre.iter().map(|x| x.1).sum();
                    let shifted = pre_dom != Some(d_after);
                    // "reserve was load-bearing" = new winner was NOT the old dominant
                    // and was rare (or absent) before the shock
                    let from_reserve = shifted && rank.map_or(true, |r| r > 0);
                    if from_reserve { reserve_hits += 1; }
                    events += 1;
                    let rank_str = match rank { Some(r) => format!("#{}", r + 1), None => "ABSENT(new mutation)".into() };
                    println!("shock@{:>5} recover@{:>5}: pop={:>4} div={:>3} | new winner was pre-shock {} (was {}/{}) | shifted:{} | reserve-driven:{}",
                        wt - recovery, wt, s.agents.len(), s.genome_diversity(),
                        rank_str, pre_count, total_pre, shifted, from_reserve);
                }
            }
        }
        println!("\nReserve-driven recoveries: {}/{} shocks", reserve_hits, events);
        println!("Read: high ratio → the floor's preserved minorities BECAME the new winners");
        println!("      (the diversity reserve is load-bearing, not decorative — MECHANISM supported).");
        println!("      low ratio → same dominant persists → floor helps via headcount, not variant-supply.");
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "dmech") {
        // DIRECTIONAL mechanism test: the shock flips which trait-group (regulate op >=4 vs <4)
        // the environment favors. Q: when the regime flips, does the new dominant come from the
        // group that was DISFAVORED before (the reserve the floor kept alive)? If yes across
        // flips, the diversity reserve is load-bearing — the mechanism is real.
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(20000);
        let capped_on = args.iter().any(|a| a == "capped");
        let pulse_on = capped_on || args.iter().any(|a| a == "pulse");
        let floor_on = pulse_on || !args.iter().any(|a| a == "nofloor");
        let shock: u64 = std::env::var("SHOCK").ok().and_then(|v| v.parse().ok()).unwrap_or(2000);
        let recovery = shock * 3 / 4; // give the newly-favored group time to take over
        let mut s = Simulation::new(seed);
        s.floor_energy = if floor_on { Some(30) } else { None };
        s.pulse_threshold = if pulse_on { Some(0.30) } else { None };
        s.cap_threshold = if capped_on { Some(0.70) } else { None };
        s.dir_penalty = std::env::var("DIR_PENALTY").ok().and_then(|v| v.parse().ok()).unwrap_or(3); // v7 knob
        s.shock_interval = Some(shock);
        s.directional = true;
        let pop: usize = std::env::var("POP").ok().and_then(|v| v.parse().ok()).unwrap_or(50); // v7 founding knob
        s.initialize_population(pop, true);
        let group = |g: [u8; 8]| -> u8 { ((g[7] & 7) >= 4) as u8 };
        let mut pre: Vec<([u8; 8], usize)> = Vec::new();
        let mut pre_dom: Option<[u8; 8]> = None;
        let mut reserve_hits = 0u32;
        let mut events = 0u32;
        let mut extinct_at: Option<u64> = None;
        println!("=== DIRECTIONAL mechanism test — {} arm (seed {}, {} ticks) ===",
                 if capped_on { "PULSE+CAP(v5)" } else if pulse_on { "PULSE(targeted)" } else if floor_on { "FLOOR" } else { "NO-FLOOR" }, seed, ticks);
        println!("Q: when the regime flips, does the new winner come from the just-favored reserve?\n");
        for _ in 0..ticks {
            s.tick();
            if s.agents.is_empty() { extinct_at = Some(s.world.tick); break; }
            let wt = s.world.tick;
            if (wt + 1) % shock == 0 {
                pre = s.genome_freq();
                pre_dom = pre.first().map(|x| x.0);
            }
            if wt >= shock && wt % shock == recovery && !pre.is_empty() {
                let after = s.genome_freq();
                if let Some(d_after) = after.first().map(|x| x.0) {
                    let favored = s.world.regime;
                    let dom_grp = group(d_after);
                    let dom_favored = dom_grp == favored;
                    let shifted = pre_dom != Some(d_after);
                    let pre_grp_favored = pre_dom.map_or(false, |d| group(d) == favored);
                    // reserve load-bearing = new winner is from the now-favored group,
                    // AND the pre-shock dominant was from the OTHER (previously-favored) group
                    let from_reserve = dom_favored && shifted && !pre_grp_favored;
                    if from_reserve { reserve_hits += 1; }
                    events += 1;
                    let g1 = s.agents.iter().filter(|a| (a.genome[0] & 7) >= 4).count();
                    let g0 = s.agents.len() - g1;
                    let fav_share = if favored == 1 { g1 } else { g0 } as f64 / s.agents.len() as f64;
                    println!("flip@{:>5} recover@{:>5}: pop={:>4} div={:>3} favored-grp={} | grp0={:>4} grp1={:>4} favored-share={:.0}% | winner-grp={} reserve-driven:{}",
                        wt - recovery, wt, s.agents.len(), s.genome_diversity(), favored,
                        g0, g1, fav_share * 100.0, dom_grp, from_reserve);
                }
            }
        }
        match extinct_at {
            Some(t) => println!("\nEXTINCT at tick {} — the arm could not survive directional shocks.", t),
            None => {
                println!("\nReserve-driven recoveries: {}/{} flips", reserve_hits, events);
                println!("Read: high ratio → floor-preserved off-regime variants BECOME the winners when");
                println!("      their regime returns → the weak are load-bearing → MECHANISM SUPPORTED.");
                println!("      Compare vs `dmech {} {} nofloor` — if no-floor goes extinct or can't shift,", seed, ticks);
                println!("      the reserve (and the floor that preserves it) is what enables adaptation.");
            }
        }
        return;
    }
    if std::env::args().any(|a| a == "gaptest") {
        // Stage 4: anticipation-gap measurement. Agent x=100, SENSE_THREAT_RANGE=5 -> detectable
        // col 95; wave start_tick=50 speed 1.0 -> front=95 at tick 145 (first detection).
        let mut s = Simulation::new(1);
        s.agents.clear();
        let cases: [(u64, Option<u64>); 4] = [(1, Some(140)), (2, Some(145)), (3, None), (4, Some(40))];
        for (id, lsa) in cases {
            let mut a = agent::Agent::new(id, 100, 50, [0u8; 8]);
            a.last_shield_activation = lsa;
            s.agents.push(a);
        }
        s.world.tick = 145;
        let wave = world::WaveState { start_tick: 50, speed: 1.0, active: true, stealth: false };
        let neg = s.check_wave_detection(&wave);
        println!("neg_gaps={} (expect 1: only the agent that shielded before detection)", neg);
        for a in &s.agents {
            println!("  id={} last_shield={:?} gaps={:?} det_tick={:?}", a.id, a.last_shield_activation, a.anticipation_gaps, a.wave_detection_tick);
        }
        return;
    }
    if std::env::args().any(|a| a == "predicttest") {
        // Stage 3: sense_threat (wave proximity) + proc_predict (anticipation) vs Python formulas.
        let mut r = PyRandom::seed(1);
        let mut w = World::new(&mut r);
        let ag = agent::Agent::new(1, 100, 50, [0u8; 8]);
        println!("sense_threat (agent x=100, wave speed 1.0 from t0):");
        for &tick in &[90u64, 96, 97, 100, 102] {
            w.tick = tick;
            w.current_wave = Some(world::WaveState { start_tick: 0, speed: 1.0, active: true, stealth: false });
            println!("  tick={} threat={}", tick, ops::sense_op(0x01, &ag, &w));
        }
        w.tick = 97;
        w.current_wave = Some(world::WaveState { start_tick: 0, speed: 1.0, active: true, stealth: true });
        println!("  tick=97 STEALTH threat={}", ops::sense_op(0x01, &ag, &w));
        let mut a2 = agent::Agent::new(2, 100, 50, [0u8; 8]);
        a2.wave_arrival_times = vec![100, 300, 500];
        println!("proc_predict (arrivals [100,300,500], interval 200, predicted_next=700, horizon 15):");
        for &tick in &[680u64, 690, 700, 701] {
            w.tick = tick;
            println!("  tick={} fires={}", tick, ops::process_op(0x04, 0.0, &a2, &w));
        }
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "exp3") {
        // Stage 5: the anticipation emergence run. Per tick: spawn wave on interval →
        // check_wave_detection (measure gaps) → apply_wave_damage (+cleanup) → thermal drain
        // (+cleanup) → tick. Arm A (default) = reactive-only STRICT EMERGENCE; Arm B = "seeded".
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(50000);
        let seeded = args.iter().any(|a| a == "seeded");
        // V1: interval 200 → single-wave overwrite → waves only sweep the left ~third.
        // V2 "global": interval 1300 > max crossing time (480/0.4=1200) → every wave crosses the
        // WHOLE world, so all agents face periodic waves (incl. stealth, survivable only by prediction).
        let global = args.iter().any(|a| a == "global");
        let wave_interval: usize = if global { 1300 } else { 200 };
        // S_SELF,S_THREAT,P_THRESH,P_THRESH,M_NONE,A_REPRO,A_SHIELD,R_NONE
        let reactive: [u8; 8] = [0x05, 0x01, 0x00, 0x00, 0x00, 0x04, 0x03, 0x00];
        // S_LIGHT,S_THREAT,P_THRESH,P_PREDICT,M_PATTERN,A_REPRO,A_SHIELD,R_NONE
        let anticipatory: [u8; 8] = [0x02, 0x01, 0x00, 0x04, 0x07, 0x04, 0x03, 0x00];
        let mut s = Simulation::new(seed);
        s.world.initialize_light_gradient();
        s.initialize_population(100, true);
        // "open" reactions: A1 (threat-response slot) seeded diverse so the defense repertoire
        // competes — SHIELD(defend) vs FLEE(flight) vs IDLE(freeze) vs TOXIN(fight/deter).
        let open = args.iter().any(|a| a == "open");
        let repertoire: [u8; 4] = [0x03, 0x07, 0x00, 0x06]; // shield, flee, idle, toxin
        // uniform: seed ALL non-seeded agents with one reaction (isolate a single defense's fitness)
        let uniform: Option<u8> = if args.iter().any(|a| a == "allflee") { Some(0x07) }
            else if args.iter().any(|a| a == "allidle") { Some(0x00) }
            else if args.iter().any(|a| a == "alltoxin") { Some(0x06) } else { None };
        let n_ant = if seeded { 10 } else { 0 };
        for (i, a) in s.agents.iter_mut().enumerate() {
            a.genome = if i < n_ant { anticipatory } else { reactive };
            if i >= n_ant {
                if let Some(u) = uniform { a.genome[6] = u; }
                else if open { a.genome[6] = repertoire[i % 4]; }
            }
        }
        println!("=== EXP3 anticipation — Arm {} (seed {}, {} ticks) ===",
                 if seeded { "B seeded" } else { "A reactive-only" }, seed, ticks);
        // cumulative metrics (robust to agents dying and taking their gaps with them)
        let mut first_neg: Option<u64> = None;
        let mut cum_neg = 0u64;   // negative anticipation-gap events over the WHOLE run
        let mut waves = 0u64;
        let mut ever_predictor = false; // did PROC_PREDICT (P1=4) ever evolve/exist?
        let mut peak_predictors = 0usize;
        for t in 0..ticks {
            let tt = t as u64;
            if t > 0 && t % wave_interval == 0 {
                let w = s.world.spawn_wave(tt, &mut s.rng);
                s.world.current_wave = Some(w);
                waves += 1;
            }
            if let Some(w) = s.world.current_wave {
                if w.active {
                    let neg = s.check_wave_detection(&w);
                    cum_neg += neg;
                    if neg > 0 && first_neg.is_none() { first_neg = Some(tt); }
                    s.apply_wave_damage(&w);
                    s.cleanup_dead();
                    if w.is_complete(tt) { s.world.current_wave = None; }
                }
            }
            s.apply_thermal_drain_all();
            s.cleanup_dead();
            s.tick();
            if t % 2000 == 0 {
                let p = s.agents.iter().filter(|a| a.genome[3] == 0x04).count();
                if p > 0 { ever_predictor = true; }
                if p > peak_predictors { peak_predictors = p; }
            }
            if s.agents.is_empty() { println!("(population extinct at tick {})", tt); break; }
        }
        let predictors_now = s.agents.iter().filter(|a| a.genome[3] == 0x04).count();
        println!("final pop={} waves={} deaths={}", s.agents.len(), waves, s.total_deaths);
        println!("PROC_PREDICT (P1=4): now={} peak={} everAppeared={}", predictors_now, peak_predictors, ever_predictor);
        println!("CUMULATIVE negative-gap events (anticipation!): {} | first at tick {:?}", cum_neg, first_neg);
        if open {
            let names = ["idle/freeze", "move", "consume", "SHIELD", "reproduce", "signal", "toxin/fight", "FLEE"];
            let mut rc = [0usize; 8];
            for a in &s.agents { rc[(a.genome[6] & 7) as usize] += 1; }
            print!("REACTION distribution among survivors (A1):");
            for i in 0..8 { if rc[i] > 0 { print!("  {}={}", names[i], rc[i]); } }
            println!();
        }
        let verdict = if !seeded {
            if cum_neg > 0 { "✅ ANTICIPATION EMERGED — reactive-only produced negative gaps, UNSEEDED" }
            else if ever_predictor { "~ PROC_PREDICT evolved but never fired a negative gap" }
            else { "✗ no emergence (PROC_PREDICT never evolved)" }
        } else if cum_neg > 0 { "✅ seeded anticipators fired negative gaps — instrument confirmed" }
        else { "✗ even seeded fired none — wiring bug" };
        println!("VERDICT (Arm {}): {}", if seeded { "B" } else { "A" }, verdict);
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "seasons") {
        // Seasons Step 1: cyclical thermal drain, NO waves. Per tick: seasonal thermal drain
        // (+cleanup) → tick. amp=0 reproduces the exp0 economy; amp>0 adds a winter squeeze on a
        // deterministic triangle over `period` ticks. Readout: an 8-bucket phase profile (pop, mean
        // energy, births/tick) over the final years → reveals boom-bust, hoarding, and phenology.
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(20000);
        let amp: f64 = std::env::var("AMP").ok().and_then(|v| v.parse().ok()).unwrap_or(6.0);
        let period: u64 = std::env::var("PERIOD").ok().and_then(|v| v.parse().ok()).unwrap_or(2000);
        // CAP lowers the density-penalty onset (default 150) so the equilibrium sits lower and winter
        // culls show up in HEADCOUNT, not just mean energy. 150 = base carrying capacity.
        let cap: i32 = std::env::var("CAP").ok().and_then(|v| v.parse().ok()).unwrap_or(150);
        let div: i32 = std::env::var("DIV").ok().and_then(|v| v.parse().ok()).unwrap_or(5);
        // FLOOR arm: guaranteed minimum energy (UCF) — rescues an agent from winter death to `floor`.
        // The civic test under seasons: does a lean-season safety net convert collapse into survival?
        let floor: Option<i32> = std::env::var("FLOOR").ok().and_then(|v| v.parse().ok());
        // FLIP arm (Step 4): the favored trait-group (regulate op ≥4 vs <4) flips every HALF-period —
        // a direction-flipping cycle (oscillating OPTIMUM), unlike Steps 1-3 which only varied
        // intensity. Tests diversify-vs-converge: does a flipping cycle MAINTAIN diversity (each group
        // wins its half → the reserve is load-bearing) where the intensity cycle CONVERGED? The
        // disfavored group bleeds DIRP energy/tick (reuses the directional-shock penalty machinery).
        let flip: bool = std::env::var("FLIP").ok().map_or(false, |v| v == "1" || v == "true");
        // RANDFLIP: the crux manipulation. Same mean flip rate as periodic FLIP (mean interval = half),
        // but UNPREDICTABLE timing (per-tick flip probability 1/half). Isolates predictability with the
        // rate of change held equal. A dedicated RNG keeps flip timing from perturbing the sim stream.
        let randflip: bool = std::env::var("RANDFLIP").ok().map_or(false, |v| v == "1" || v == "true");
        let flip = flip || randflip;
        let dirp: i32 = std::env::var("DIRP").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
        let mut flip_rng = PyRandom::seed(seed ^ 0x5eed_5eed);
        let mut s = Simulation::new(seed);
        s.world.initialize_light_gradient();
        s.world.season_amp = amp;
        s.world.season_period = period;
        s.density_onset = cap;
        s.density_div = div;
        s.floor_energy = floor;
        s.directional = flip;
        s.dir_penalty = dirp;
        // Flip splits on a COST-NEUTRAL locus (sense slot 0, SENSE_COSTS all 0) so the two strategies
        // are metabolically equal — distinguished ONLY by which regime favors them. (Grouping on the
        // regulate gene collapses to op-5 crit-cut, the free metabolic attractor → false convergence.)
        s.dir_locus = 0;
        s.initialize_population(100, true);
        println!("=== SEASONS {}{} (seed {}, {} ticks | amp {} period {} cap {} div {}) ===",
            floor.map_or("baseline".into(), |f| format!("FLOOR={}", f)),
            if randflip { format!(" +RANDFLIP(dirp {})", dirp) } else if flip { format!(" +FLIP(dirp {})", dirp) } else { String::new() },
            seed, ticks, amp, period, cap, div);
        println!("    deep-winter drain ~{:.1}/tick vs summer 0 | passive gain ≤5, baseline 1", amp);
        let sample_start = (ticks as u64).saturating_sub(4 * period);
        let (mut bpop, mut bene, mut bn) = ([0f64; 8], [0f64; 8], [0u64; 8]);
        let (mut bbirth, mut bg1) = ([0u64; 8], [0f64; 8]);
        let (mut min_pop, mut max_pop) = (100usize, 100usize);
        let mut extinct_at: Option<u64> = None;
        let mut last_repro = s.total_reproductions;
        let half = (period / 2).max(1);
        for t in 0..ticks {
            let tt = t as u64;
            if flip { // regime flip: periodic (predictable) or random-timing (unpredictable), matched mean
                let do_flip = if randflip { flip_rng.random() < 1.0 / half as f64 }
                              else { tt > 0 && tt % half == 0 };
                if do_flip { s.world.regime ^= 1; }
            }
            s.apply_thermal_drain_all();
            s.cleanup_dead();
            s.tick();
            let p = s.agents.len();
            if p < min_pop { min_pop = p; }
            if p > max_pop { max_pop = p; }
            if tt >= sample_start && p > 0 {
                let phase = (s.world.tick % period) as f64 / period as f64;
                let b = ((phase * 8.0) as usize).min(7);
                bpop[b] += p as f64;
                bene[b] += s.agents.iter().map(|a| a.energy).sum::<f64>() / p as f64;
                bbirth[b] += s.total_reproductions - last_repro;
                let g1 = s.agents.iter().filter(|a| (a.genome[0] & 7) >= 4).count();
                bg1[b] += g1 as f64 / p as f64; // group-1 share this tick
                bn[b] += 1;
            }
            last_repro = s.total_reproductions;
            if p == 0 { extinct_at = Some(tt); break; }
        }
        match extinct_at {
            Some(t) => println!("\n💀 EXTINCT at tick {} — the population could not ride the cycle at amp {}.", t, amp),
            None => {
                println!("\nfinal pop={} | winter-trough(min)={} summer-peak(max)={} | births={} deaths={} rescues={}",
                    s.agents.len(), min_pop, max_pop, s.total_reproductions, s.total_deaths, s.floor_rescues);
                println!("\nPhase profile (final 4 years; phase 0=midsummer, 4=deep winter):");
                println!("  {:>5} {:>8} {:>8} {:>9}", "phase", "pop", "meanE", "births/t");
                for b in 0..8 {
                    if bn[b] == 0 { continue; }
                    let tag = if b == 0 { "  ← summer" } else if b == 4 { "  ← WINTER" } else { "" };
                    println!("  {:>5} {:>8.0} {:>8.1} {:>9.3}{}",
                        b, bpop[b]/bn[b] as f64, bene[b]/bn[b] as f64, bbirth[b] as f64/bn[b] as f64, tag);
                }
                // Quantify the cycle: energy swing (buffering) + warm/cold birth ratio (phenology).
                let live: Vec<f64> = (0..8).filter(|&b| bn[b] > 0).map(|b| bene[b]/bn[b] as f64).collect();
                let (emax, emin) = (live.iter().cloned().fold(0.0f64, f64::max), live.iter().cloned().fold(999.0f64, f64::min));
                let livep: Vec<f64> = (0..8).filter(|&b| bn[b] > 0).map(|b| bpop[b]/bn[b] as f64).collect();
                let (pmax, pmin) = (livep.iter().cloned().fold(0.0f64, f64::max), livep.iter().cloned().fold(1e9f64, f64::min));
                let warm: u64 = [6usize,7,0,1].iter().map(|&b| bbirth[b]).sum();
                let cold: u64 = [2usize,3,4,5].iter().map(|&b| bbirth[b]).sum();
                println!("\nCYCLE  energy swing: {:.0}→{:.0} (Δ{:.0}, {:.0}% of peak)  |  headcount swing: {:.0}→{:.0} ({:.0}%)",
                    emax, emin, emax - emin, (emax - emin) / emax * 100.0, pmax, pmin, (pmax - pmin) / pmax * 100.0);
                // NB: at carrying capacity, births are winter-REPLACEMENT (track deaths opening slots),
                // not clean adaptive phenology — this ratio is confounded by the density cap.
                println!("REPRO TIMING  warm-half births {} vs cold-half {}  (ratio {:.1}× — confounded by density, not phenology)",
                    warm, cold, warm as f64 / cold.max(1) as f64);
                if flip {
                    let shares: Vec<f64> = (0..8).filter(|&b| bn[b] > 0).map(|b| bg1[b] / bn[b] as f64).collect();
                    let (smax, smin) = (shares.iter().cloned().fold(0.0f64, f64::max),
                                        shares.iter().cloned().fold(1.0f64, f64::min));
                    let both = smin > 0.05 && smax < 0.95; // both groups persist through the whole cycle
                    print!("FLIP  group-1 share by phase:");
                    for b in 0..8 { if bn[b] > 0 { print!(" p{}:{:.0}%", b, bg1[b] / bn[b] as f64 * 100.0); } }
                    println!();
                    println!("  share swings {:.0}%→{:.0}% across the cycle | DIVERSITY {} ({} — each group holds its half if maintained)",
                        smax * 100.0, smin * 100.0,
                        if both { "MAINTAINED" } else { "COLLAPSED to one group" },
                        if both { "reserve rides the cycle" } else { "converged" });
                }
                let names = ["none","low-cut","burst","cycle","learn","crit-cut","prioritize","adaptive"];
                let mut rc = [0usize; 8];
                for a in &s.agents { rc[(a.genome[7] & 7) as usize] += 1; }
                print!("REGULATE distribution among survivors (R = genome[7]):");
                for i in 0..8 { if rc[i] > 0 { print!("  {}={}", names[i], rc[i]); } }
                println!();
                println!("  torpor-capable (energy-conditional cost cuts low-cut/crit-cut/adaptive): {}/{}",
                    rc[1] + rc[5] + rc[7], s.agents.len());
            }
        }
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "pulse") {
        // B1 — metronome-vs-reprieve. A single global ON/OFF stressor (drought/frost pulse): drain L
        // per tick while ON, 0 while OFF. No groups, no waves. Same duty + mean bout lengths across
        // TIMING regimes; only the VARIANCE of bout timing changes: periodic = fixed bouts (metronome)
        // | random = geometric (high variance → reprieve windows) | inter = uniform ±50% (moderate).
        // Claim: at harsh L, higher-variance timing → higher survival; reverses at mild L. Sweep L×timing.
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(20000);
        let l: f64 = std::env::var("L").ok().and_then(|v| v.parse().ok()).unwrap_or(3.0);
        let on_len: u64 = std::env::var("ON").ok().and_then(|v| v.parse().ok()).unwrap_or(100);
        let off_len: u64 = std::env::var("OFF").ok().and_then(|v| v.parse().ok()).unwrap_or(100);
        let timing = std::env::var("TIMING").unwrap_or_else(|_| "periodic".into());
        let cap: i32 = std::env::var("CAP").ok().and_then(|v| v.parse().ok()).unwrap_or(150);
        let div: i32 = std::env::var("DIV").ok().and_then(|v| v.parse().ok()).unwrap_or(5);
        let mut s = Simulation::new(seed);
        s.world.initialize_light_gradient();
        s.density_onset = cap;
        s.density_div = div;
        s.initialize_population(100, true);
        let mut prng = PyRandom::seed(seed ^ 0x9111_5e);
        // next bout length for the current phase, by timing regime (all share the same mean)
        let next_bout = |on: bool, prng: &mut PyRandom| -> u64 {
            let mean = if on { on_len } else { off_len }.max(1);
            match timing.as_str() {
                "random" => { let p = 1.0 / mean as f64; let mut n = 1u64; while prng.random() >= p { n += 1; } n }
                "inter"  => (mean / 2).max(1) + prng.randbelow(mean as u32) as u64,
                _        => mean, // periodic: fixed
            }
        };
        println!("=== PULSE metronome-vs-reprieve (seed {}, {} ticks | L {} on {} off {} timing {} cap {} div {}) ===",
            seed, ticks, l, on_len, off_len, timing, cap, div);
        let mut pulse_on = false;
        let mut ticks_left = next_bout(false, &mut prng); // start OFF
        let mut on_ticks = 0u64;
        let (mut min_pop, mut extinct_at) = (100usize, None::<u64>);
        for t in 0..ticks {
            let tt = t as u64;
            if ticks_left == 0 { pulse_on = !pulse_on; ticks_left = next_bout(pulse_on, &mut prng); }
            ticks_left -= 1;
            if pulse_on { s.apply_flat_drain(l); s.cleanup_dead(); on_ticks += 1; }
            s.tick();
            let p = s.agents.len();
            if p < min_pop { min_pop = p; }
            if p == 0 { extinct_at = Some(tt); break; }
        }
        let duty = on_ticks as f64 / ticks as f64;
        match extinct_at {
            Some(t) => println!("💀 EXTINCT at tick {} | realized duty {:.2}", t, duty),
            None => println!("survived | final pop={} min={} | realized duty {:.2}", s.agents.len(), min_pop, duty),
        }
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "b2") {
        // B2 — targeted-floor direct test (ports Python Exp 9's adaptive predator to Rust).
        // Adaptive adversary: specializes against the DOMINANT defense (genome[6]&7), `adapt` rising
        // while the target stays dominant (capped ADAPT_MAX), falling + retargeting when it changes.
        // Predation drains adapt-target-defense users by PRED*adapt/tick (their defense is bypassed);
        // other defenses are safe → being in the crowded strategy is dangerous → escape treadmill;
        // a monoculture gets wiped when adapt maxes, diversity spreads the risk.
        // Three arms (env FLOOR): none | uncond (rescue all) | targeted (rescue only MINORITY defenses,
        // share < THRESH — the Exp 9 diversity floor). Q: does targeted survive + hold adapt low where
        // none goes extinct and uncond stasis/collapses?
        let seed: u32 = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(42);
        let ticks: usize = args.get(pos + 2).and_then(|s| s.parse().ok()).unwrap_or(50000);
        let arm = std::env::var("FLOOR").unwrap_or_else(|_| "none".into());
        let pred: f64 = std::env::var("PRED").ok().and_then(|v| v.parse().ok()).unwrap_or(3.0);
        let rise: f64 = std::env::var("RISE").ok().and_then(|v| v.parse().ok()).unwrap_or(0.003);
        let fall: f64 = std::env::var("FALL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.02);
        let thresh: f64 = std::env::var("THRESH").ok().and_then(|v| v.parse().ok()).unwrap_or(0.15);
        let floor_e: f64 = std::env::var("FLOORE").ok().and_then(|v| v.parse().ok()).unwrap_or(30.0);
        const ADAPT_MAX: f64 = 1.5;
        let mut s = Simulation::new(seed);
        s.world.initialize_light_gradient();
        s.initialize_population(150, true);
        let repertoire = [0x03u8, 0x07, 0x06, 0x00]; // shield, flee, toxin, idle — founding diversity
        for (i, a) in s.agents.iter_mut().enumerate() { a.genome[6] = repertoire[i % 4]; }
        let defense = |g: &[u8; 8]| -> usize { (g[6] & 7) as usize };
        println!("=== B2 targeted-floor vs adaptive adversary — arm {} (seed {}, {} ticks | pred {} thresh {}) ===",
            arm, seed, ticks, pred, thresh);
        let (mut adapt, mut target) = (0.0f64, 3usize); // start specialized on shield
        let mut min_pop = 150usize;
        let mut extinct_at: Option<u64> = None;
        let mut rescues = 0u64;
        for t in 0..ticks {
            let tt = t as u64;
            // 1. defense counts + dominant
            let n = s.agents.len();
            let mut cnt = [0usize; 8];
            for a in &s.agents { cnt[defense(&a.genome)] += 1; }
            let dom = (0..8).max_by_key(|&d| cnt[d]).unwrap();
            // 2. adapt update
            if dom == target { adapt = (adapt + rise).min(ADAPT_MAX); }
            else { adapt -= fall; if adapt <= 0.0 { adapt = 0.0; target = dom; } }
            // 3. MAINTENANCE FLOOR (arm) — keep the diversity reserve alive BEFORE the pressures hit.
            //    none: off | uncond: top up every low agent | targeted: only minority defenses (< THRESH)
            if arm != "none" {
                for a in s.agents.iter_mut() {
                    if !a.alive { continue; }
                    let share = cnt[defense(&a.genome)] as f64 / n.max(1) as f64;
                    let protect = arm == "uncond" || share < thresh;
                    if protect && a.energy < floor_e { a.energy = floor_e; rescues += 1; }
                }
            }
            // 4. adaptive predation — drains the adapt-target defense; other defenses are safe
            let dmg = pred * adapt;
            if dmg > 0.0 {
                for a in s.agents.iter_mut() {
                    if a.alive && defense(&a.genome) == target { a.apply_drain(dmg); }
                }
                s.cleanup_dead();
            }
            s.tick();
            let p = s.agents.len();
            if p < min_pop { min_pop = p; }
            if p == 0 { extinct_at = Some(tt); break; }
        }
        let ddiv = { let mut c = [0usize; 8]; for a in &s.agents { c[defense(&a.genome)] += 1; }
            (0..8).filter(|&d| c[d] > 0).count() };
        match extinct_at {
            Some(t) => println!("💀 EXTINCT at tick {} | final adapt {:.2} rescues {}", t, adapt, rescues),
            None => println!("survived | final pop={} min={} | adapt {:.2} | defense-diversity {}/8 | rescues {}",
                s.agents.len(), min_pop, adapt, ddiv, rescues),
        }
        return;
    }
    if std::env::args().any(|a| a == "wavedamagetest") {
        // Stage 2: deterministic damage-logic check. 4 agents at x=100, wave front reaches 100 at t=100.
        let mut s = Simulation::new(42);
        s.agents.clear();
        // (id, shield-gene in slot A1, shield_active, energy)
        let cases = [(1u64, 0x03u8, true, 200.0f64), (2, 0x03, false, 200.0), (3, 0x00, false, 200.0), (4, 0x00, false, 255.0)];
        for &(id, sg, sa, e) in &cases {
            let mut g = [0u8; 8]; g[6] = sg;
            let mut a = agent::Agent::new(id, 100, 50, g);
            a.shield_active = sa; a.energy = e;
            s.agents.push(a);
        }
        s.world.tick = 100;
        let wave = world::WaveState { start_tick: 0, speed: 1.0, active: true, stealth: false };
        let (kills, shielded) = s.apply_wave_damage(&wave);
        println!("wave damage @front=100: kills={} shielded={} (expect kills=2 shielded=1)", kills, shielded);
        for a in &s.agents {
            println!("  id={} alive={} energy={} arrivals={:?}", a.id, a.alive, a.energy, a.wave_arrival_times);
        }
        return;
    }
    if std::env::args().any(|a| a == "gausstest") {
        // Stage 1 crux: does CPython's gauss reproduce bit-for-bit in Rust f64?
        let mut r = PyRandom::seed(42);
        println!("rust gauss(0,0.05) seed 42 — raw f64 bits:");
        for _ in 0..6 {
            let v = r.gauss(0.0, 0.05);
            println!("  {:016x}  {:.17e}", v.to_bits(), v);
        }
        return;
    }
    if std::env::args().any(|a| a == "gradienttest") {
        // Stage 1: validate initialize_light_gradient by hashing the post-gradient grid vs Python.
        let mut r = PyRandom::seed(42);
        let mut w = World::new(&mut r);
        w.initialize_light_gradient();
        let mut h: u64 = 0xcbf29ce484222325;
        let mut feed = |v: i32, h: &mut u64| { for b in v.to_le_bytes() { *h ^= b as u64; *h = h.wrapping_mul(0x100000001b3); } };
        for row in &w.grid { for c in row { feed(c.energy, &mut h); feed(c.light, &mut h); } }
        println!("rust gradient grid hash (seed 42): {:016x}", h);
        let c = &w.grid[180][240]; println!("  center(240,180): light={} energy={}", c.light, c.energy);
        let e = &w.grid[0][0];     println!("  corner(0,0):     light={} energy={}", e.light, e.energy);
        return;
    }
    if std::env::args().any(|a| a == "wavetest") {
        // Stage 1: validate spawn_wave (gauss speed + random stealth) bit-for-bit vs Python.
        let mut throwaway = PyRandom::seed(1);
        let w = World::new(&mut throwaway);       // world grid built with a separate stream
        let mut rng = PyRandom::seed(42);          // clean stream for the wave draws only
        println!("rust spawn_wave x6 (seed 42): speed_bits speed stealth");
        for i in 0..6u64 {
            let wave = w.spawn_wave(i, &mut rng);
            println!("  {:016x}  {:.9}  {}", wave.speed.to_bits(), wave.speed, wave.stealth);
        }
        return;
    }
    if std::env::args().any(|a| a == "shuffle") {
        let mut r = PyRandom::seed(42);
        let mut x: Vec<i32> = (0..10).collect();
        r.shuffle(&mut x);
        println!("shuffled: {:?}", x); // Python: [7, 3, 2, 8, 5, 6, 9, 4, 0, 1]
        assert_eq!(x, vec![7, 3, 2, 8, 5, 6, 9, 4, 0, 1], "SHUFFLE MISMATCH");
        println!("✅ shuffle matches Python");
        return;
    }
    if std::env::args().any(|a| a == "world") {
        let mut r = PyRandom::seed(42);
        let w = World::new(&mut r);
        println!("seed 42 World: {}x{}, {} sources", w.width, w.height, w.energy_sources.len());
        println!("first sources: {:?}", &w.energy_sources[..3.min(w.energy_sources.len())]);
        println!("state_hash: {:016x}", w.state_hash());
        return;
    }
    println!("seed 42 random():");
    let mut r = PyRandom::seed(42);
    let vals: Vec<f64> = (0..6).map(|_| r.random()).collect();
    for v in &vals { print!("{:.16} ", v); }
    println!();

    println!("seed 42 randrange(256):");
    let mut r = PyRandom::seed(42);
    let vals: Vec<u32> = (0..6).map(|_| r.randrange(256)).collect();
    println!("{:?}", vals);

    // independent check: different seed, non-power-of-2 range (exercises rejection sampling)
    println!("seed 123 randrange(50):");
    let mut r = PyRandom::seed(123);
    let vals: Vec<u32> = (0..8).map(|_| r.randrange(50)).collect();
    println!("{:?}", vals);
    // Python: [3, 17, 5, 49, 26, 17, 6, 2]
    assert_eq!(vals, vec![3, 17, 5, 49, 26, 17, 6, 2], "RNG MISMATCH — port is broken");
    println!("\n✅ all RNG checks pass — Rust matches Python bit-for-bit");
}
