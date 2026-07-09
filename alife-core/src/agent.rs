// Agent — full port including per-tick state used by the tick loop.
use crate::rng::PyRandom;
use crate::world::World;

pub const GENOME_LENGTH: usize = 8;
pub const INITIAL_ENERGY: i32 = 200;
pub const INITIAL_POPULATION: usize = 50;
pub const ENERGY_MAX: i32 = 255;
pub const REPRODUCTION_THRESHOLD: i32 = 120;
pub const REPRODUCTION_COST: i32 = 80;
pub const CRITICAL_LOW_ENERGY: i32 = 50;
pub const BURST_THRESHOLD: i32 = 150;
pub const CONSUME_AMOUNT: i32 = 25;
pub const BASELINE_DRAIN: i32 = 1;
// costs from config.py
pub const SENSE_COSTS: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
pub const PROCESS_COSTS: [i32; 8] = [0, 0, 1, 1, 2, 1, 1, 0];
pub const MEMORY_COSTS: [i32; 8] = [0, 0, 1, 1, 0, 0, 1, 2];
pub const ACT_COSTS: [i32; 8] = [0, 1, 0, 1, 2, 1, 2, 1];
pub const REGULATE_COSTS: [i32; 8] = [0, 0, 1, 1, 1, 0, 1, 2];

pub struct Agent {
    pub id: u64,
    pub generation: i32,
    pub x: i32,
    pub y: i32,
    pub genome: [u8; GENOME_LENGTH],
    pub energy: f64, // f64 for exp3 fractional thermal drain; integer-valued in base (hash casts to i32)
    pub alive: bool,
    pub age: i32,
    pub memory: Vec<f64>, // sense values (f64: sense_self can be fractional in exp3)
    pub pattern_memory: Vec<(i32, i32)>,
    pub shield_active: bool,
    pub signaling: bool,
    pub toxin_active: bool,
    pub op_usage: Vec<(u8, i32)>, // insertion-ordered (reg_learn iterates it)
    pub reproduction_cooldown: i32,
    pub wave_arrival_times: Vec<i32>, // empty in exp0
    // exp3 (anticipation) state — all inert unless waves run (base hash unaffected)
    pub last_shield_activation: Option<u64>, // tick the shield last fired
    pub wave_detected: Option<u64>,          // start_tick of the wave already detected (key)
    pub wave_detection_tick: Option<u64>,    // tick this agent first sensed the current wave
    pub anticipation_gaps: Vec<i64>,         // shield_tick - detection_tick; <0 = anticipatory
    pub clock: u32,                          // B3 (entrainment): endogenous phase counter (0 in base/exp3)
}

impl Agent {
    pub fn new(id: u64, x: i32, y: i32, genome: [u8; GENOME_LENGTH]) -> Self {
        Agent {
            id, generation: 0, x, y, genome, energy: INITIAL_ENERGY as f64, alive: true,
            age: 0, memory: Vec::new(), pattern_memory: Vec::new(),
            shield_active: false, signaling: false, toxin_active: false,
            op_usage: Vec::new(), reproduction_cooldown: 0, wave_arrival_times: Vec::new(),
            last_shield_activation: None, wave_detected: None, wave_detection_tick: None,
            anticipation_gaps: Vec::new(), clock: 0,
        }
    }
    // genome slot accessors
    pub fn sense_ops(&self) -> (u8, u8) { (self.genome[0], self.genome[1]) }
    pub fn process_ops(&self) -> (u8, u8) { (self.genome[2], self.genome[3]) }
    pub fn memory_op(&self) -> u8 { 0 } // MEMORY_ENABLED=false in config -> MEM_NONE forced
    pub fn act_ops(&self) -> (u8, u8) { (self.genome[5], self.genome[6]) }
    pub fn regulate_op(&self) -> u8 { self.genome[7] }

    pub fn apply_energy_cost(&mut self, cost: i32) {
        self.energy -= cost as f64;
        if self.energy <= 0.0 { self.energy = 0.0; self.alive = false; }
    }
    /// exp3: apply a fractional (thermal) drain directly.
    pub fn apply_drain(&mut self, drain: f64) {
        self.energy -= drain;
        if self.energy <= 0.0 { self.energy = 0.0; self.alive = false; }
    }
    pub fn add_energy(&mut self, amount: i32) {
        self.energy = (self.energy + amount as f64).min(ENERGY_MAX as f64);
    }
    pub fn tick_age(&mut self) { self.age += 1; }
    pub fn reset_tick_state(&mut self) {
        self.shield_active = false; self.signaling = false; self.toxin_active = false;
    }
    pub fn record_op_usage(&mut self, op: u8) {
        for e in self.op_usage.iter_mut() { if e.0 == op { e.1 += 1; return; } }
        self.op_usage.push((op, 1));
    }
}

// (Removed the standalone `initialize_population` free fn 2026-07: it duplicated
// Simulation::initialize_population — the only real init path — and was used solely by the `pop`
// test, which now calls the canonical method. One init path, no drift.)

/// Population hash — canonical path validated bit-for-bit both langs: `2f59d3550af7cf2f`
/// (seed 42, 50 agents; Rust == Python 2026-07). Also folded into `Simulation::state_hash`.
pub fn population_hash(agents: &[Agent]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut feed = |v: i32, h: &mut u64| {
        for b in v.to_le_bytes() { *h ^= b as u64; *h = h.wrapping_mul(0x100000001b3); }
    };
    for a in agents {
        feed(a.x, &mut h); feed(a.y, &mut h);
        for &g in &a.genome { feed(g as i32, &mut h); }
        feed(a.energy as i32, &mut h); feed(a.generation, &mut h); feed(a.age, &mut h);
    }
    h
}
