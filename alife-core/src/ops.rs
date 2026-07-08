// The 40 ops — verbatim port of ops.py, exp0 (no-wave) paths. All deterministic.
use crate::agent::{Agent, CONSUME_AMOUNT, CRITICAL_LOW_ENERGY, BURST_THRESHOLD,
                   REPRODUCTION_THRESHOLD};
use crate::world::{World, SENSE_THREAT_RANGE};

const SIGNAL_ACTIVE: bool = false; // base sim (exp0); set true only in exp5
const TOXIN_ACTIVE: bool = false;

// ---- SENSE (agent, world) -> f64 (sense_self carries fractional energy in exp3) ----
pub fn sense_op(code: u8, a: &Agent, w: &World) -> f64 {
    if code & 0x07 == 5 { a.energy.min(255.0) } else { sense_op_int(code, a, w) as f64 }
}
fn sense_op_int(code: u8, a: &Agent, w: &World) -> i32 {
    match code & 0x07 {
        0 => w.energy_at(a.x, a.y),
        1 => sense_threat(a, w), // exp3: wave-front proximity (falls back to cell threat when no wave)
        2 => w.light_at(a.x, a.y),
        3 => { // neighbor: avg of 8-adjacent cell energy (// count)
            let (mut total, mut count) = (0, 0);
            for dx in -1..=1 { for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let (nx, ny) = (a.x + dx, a.y + dy);
                if w.in_bounds(nx, ny) { total += w.energy_at(nx, ny); count += 1; }
            }}
            if count > 0 { total / count } else { 0 }
        }
        4 => { // density: 3x3 (INCLUDING center) occupied count * 28, cap 255
            let mut count = 0;
            for dx in -1..=1 { for dy in -1..=1 {
                let (nx, ny) = (a.x + dx, a.y + dy);
                if w.in_bounds(nx, ny) && w.occupied(nx, ny) { count += 1; }
            }}
            (count * 28).min(255)
        }
        5 => a.energy.min(255.0) as i32, // unused (sense_op intercepts 5); kept for completeness
        6 => { // gradient: dir*32 of best adjacent energy, 255 if flat
            let mut best_dir = 255;
            let mut best_e = w.energy_at(a.x, a.y);
            let dirs = [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)];
            for (i, &(dx, dy)) in dirs.iter().enumerate() {
                let (nx, ny) = (a.x + dx, a.y + dy);
                if w.in_bounds(nx, ny) {
                    let e = w.energy_at(nx, ny);
                    if e > best_e { best_e = e; best_dir = (i as i32) * 32; }
                }
            }
            best_dir
        }
        _ => a.age.min(255), // 7
    }
}

/// exp3: threat = wave-front proximity (0 far → 255 at/past agent). Stealth waves read 0
/// (undetectable — only prediction survives them). Falls back to cell threat when no wave.
fn sense_threat(a: &Agent, w: &World) -> i32 {
    if let Some(wave) = &w.current_wave {
        if wave.active && !wave.stealth {
            let front = wave.front_position(w.tick);
            let distance = front - a.x as f64; // negative => wave hasn't reached agent yet (L→R)
            if distance >= 0.0 {
                return 255; // wave at or past the agent
            }
            let proximity = SENSE_THREAT_RANGE as f64 + distance;
            if proximity > 0.0 {
                let threat = ((proximity / SENSE_THREAT_RANGE as f64) * 255.0) as i32;
                if threat > 0 { return threat; }
            }
        }
    }
    w.threat_at(a.x, a.y) // fallback (0 in base/exp0)
}

// ---- PROCESS (sense_value, agent, world) -> bool ----
pub fn process_op(code: u8, sv: f64, a: &Agent, w: &World) -> bool {
    match code & 0x07 {
        0 => sv > 128.0,
        1 => sv > a.energy,
        2 => a.memory.last().map_or(false, |&m| sv > m),
        3 => a.memory.len() >= 2 && a.memory[a.memory.len()-1] > a.memory[a.memory.len()-2],
        4 => { // predict: fire when the next wave (extrapolated from arrival history) is within horizon
            if a.wave_arrival_times.len() < 2 { false }
            else {
                let t = &a.wave_arrival_times;
                let mut sum = 0i64;
                for i in 1..t.len() { sum += (t[i] - t[i - 1]) as i64; }
                let avg_interval = sum as f64 / (t.len() - 1) as f64;
                let predicted_next = *t.last().unwrap() as f64 + avg_interval;
                let ticks_until = predicted_next - w.tick as f64;
                0.0 < ticks_until && ticks_until < (SENSE_THREAT_RANGE * 3) as f64
            }
        }
        5 => sv > 128.0, // beat: exp0 fallback (no waves) -> sense>128
        6 => { // average
            if a.memory.is_empty() { sv > 128.0 }
            else { (a.memory.iter().sum::<f64>() / a.memory.len() as f64) > 128.0 }
        }
        _ => sv < 64.0, // invert
    }
}

// ---- MEMORY (agent, sense_value) mutates ----
pub fn memory_op(code: u8, a: &mut Agent, sv: f64) {
    match code & 0x07 {
        0 => a.memory.clear(),
        1 => a.memory = vec![sv],
        2 => { a.memory.push(sv); if a.memory.len() > 4 { a.memory.remove(0); } }
        3 => { a.memory.push(sv); if a.memory.len() > 8 { a.memory.remove(0); } }
        4 => if a.memory.is_empty() || sv > a.memory[0] { a.memory = vec![sv]; },
        5 => if a.memory.is_empty() || sv < a.memory[0] { a.memory = vec![sv]; },
        6 | 7 => if sv > 128.0 { // pattern / dual (same for exp0)
            a.pattern_memory.push((a.age, sv as i32));
            if a.pattern_memory.len() > 4 { a.pattern_memory.remove(0); }
        },
        _ => {}
    }
}

// ---- ACT (agent, world) -> requested_reproduction:bool ----
pub fn act_op(code: u8, a: &mut Agent, w: &mut World) -> bool {
    match code & 0x07 {
        0 => {} // idle
        1 => { // move toward best adjacent energy if empty
            let (mut bx, mut by, mut be) = (a.x, a.y, w.energy_at(a.x, a.y));
            for dx in -1..=1 { for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let (nx, ny) = (a.x + dx, a.y + dy);
                if w.in_bounds(nx, ny) && !w.occupied(nx, ny) {
                    let e = w.energy_at(nx, ny);
                    if e > be { be = e; bx = nx; by = ny; }
                }
            }}
            if (bx, by) != (a.x, a.y) {
                if w.move_occupant(a.id, a.x, a.y, bx, by) { a.x = bx; a.y = by; }
            }
        }
        2 => { // consume
            let avail = w.energy_at(a.x, a.y);
            let amt = avail.min(CONSUME_AMOUNT);
            a.add_energy(amt);
            w.reduce_energy(a.x, a.y, amt);
        }
        3 => a.shield_active = true,
        4 => { // reproduce: request if eligible
            return a.reproduction_cooldown == 0 && a.energy >= REPRODUCTION_THRESHOLD as f64;
        }
        5 => if SIGNAL_ACTIVE { a.signaling = true; },
        6 => if TOXIN_ACTIVE { a.toxin_active = true; },
        7 => { // flee: move opposite threat direction
            if let Some((tx, ty)) = w.threat_direction(a.x, a.y) {
                let fx = (a.x - (tx - a.x)).max(0).min(w.width - 1);
                let fy = (a.y - (ty - a.y)).max(0).min(w.height - 1);
                if !w.occupied(fx, fy) {
                    if w.move_occupant(a.id, a.x, a.y, fx, fy) { a.x = fx; a.y = fy; }
                }
            }
        }
        _ => {}
    }
    false
}

// ---- REGULATE (agent) -> (nonempty, cost_modifier) ----
pub fn regulate_op(code: u8, a: &Agent) -> (bool, i32) {
    match code & 0x07 {
        0 => (false, 0),
        1 => if a.energy < CRITICAL_LOW_ENERGY as f64 { (true, -1) } else { (false, 0) },
        2 => if a.energy > BURST_THRESHOLD as f64 { (true, 0) } else { (false, 0) },
        3 => (true, 0),                 // cycle: always has behavior_mode key
        4 => (true, 0),                 // learn: always has op_discounts key
        5 => if a.energy < (CRITICAL_LOW_ENERGY / 2) as f64 { (true, 0) } else { (false, 0) },
        6 => (true, 0),                 // prioritize: always nonempty
        _ => (true, if a.energy < CRITICAL_LOW_ENERGY as f64 { -1 } else { 0 }), // adaptive
    }
}
