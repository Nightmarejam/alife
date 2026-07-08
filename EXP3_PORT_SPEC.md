# EXP3 (Anticipation) — Rust Port Spec

*Written 2026-07 before any Rust code, the way TICK_LOOP_PORT_SPEC.md preceded the base port.
This is the written foundation: exact mechanics, RNG order, staged validation gates, and a
sharpened experiment design that makes the claim falsifiable.*

## Honest status (why this port exists)

The "89.2% predictive shielding / genuine anticipation" result is **asserted, not confirmed**
(WORLD_DESIGN_LINEAGE.md): the original run left no reproducibility receipt. This port is not a
reproduction — it is the **instrument that will confirm, refute, or reframe** the anticipation
claim with a deterministic receipt. Target tier on success: `anticipation` → CONFIRMED vocabulary.

## The sharpened experiment design (STRICT EMERGENCE)

The Python exp3 *seeds 10/100 agents with an explicit anticipatory genome* (`PROC_PREDICT` +
`MEM_PATTERN`). That confounds "emergence" with "spread of a seeded trait." We run **two arms**:

- **Arm A — reactive-only control (THE emergence test):** all founders reactive (`SEEDED_GENOME`,
  `MEM_NONE`). Anticipation can only appear via mutation (`MEM_NONE→MEM_PATTERN`,
  `PROC_THRESHOLD→PROC_PREDICT`). **Success = negative anticipation gaps appear here, reproducibly
  across N seeds.** This is the claim worth a word: foresight arising from simple rules, unseeded.
- **Arm B — seeded diagnostic:** the original 10 anticipatory + 90 reactive. Confirms the
  anticipatory genome *can* produce negative gaps and measures how it spreads. Diagnostic only.

If negative gaps emerge in Arm A → `anticipation` CONFIRMED (strict). If only Arm B shows them →
honest reframe: "anticipation is selectable but was not observed to emerge unseeded here."

## The measurement instrument (the anticipation gap)

From `simulation.py:check_wave_detection`. Per agent, on **first** detection of a wave
(`wave_key = (start_tick, direction)`): a wave is *detectable* when its front reaches
`agent.x - SENSE_THREAT_RANGE` (L→R). If the agent's `last_shield_activation >= wave.start_tick`:

```
gap = last_shield_activation - detection_tick
  gap > 0  reactive     (shield after it could sense)
  gap = 0  simultaneous
  gap < 0  ANTICIPATORY  (shield BEFORE it could sense) ← the result
```

Requires agent state: `last_shield_activation`, `wave_detected` (key), `wave_detection_tick`,
`anticipation_gaps: Vec<i64>`, `wave_arrival_times`.

## The bit-exactness risk — RESOLVED (kept for the record)

**RESOLVED 2026-07 (Stage 1).** Tested directly: CPython `gauss` and `spawn_wave` (speed + stealth)
reproduce **bit-for-bit** in Rust `f64` on the Mac dev machine (shared libm) — 6/6 draws identical.
The wave layer stays FULLY bit-exact; the statistical fallback below is **not needed** (kept only
for a future cross-machine/Linux case where libm could differ). Receipts: `alife-core gausstest`
and `alife-core wavetest` vs Python. The original risk analysis follows.



`world.py:spawn_wave` sets `speed = WAVE_SPEED_C * (1 + random.gauss(0, WAVE_SPEED_VARIANCE))`,
clamped to [0.4, 1.6]. **`random.gauss` uses transcendentals (`log`, `cos`, `sin`, `sqrt`) and a
cached second value** — Rust `f64` `ln/cos/sin` are not guaranteed bit-identical to CPython's
libm. A 1e-15 speed difference shifts `front_position`, which can flip the `abs(agent.x-front)<1.0`
hit test → cascade. So the integer-exact strategy that validated the base sim (`a2bb005395f79766`)
**may not extend to the wave layer.**

**Validation strategy (adjusted, honest):**
1. First, replicate CPython `gauss` exactly (same algorithm + `gauss_next` caching) and **test the
   transcendental agreement** for our seeds. If `speed` matches to the bit → keep full bit-exact
   validation.
2. If it diverges → downgrade *only the wave-speed layer* to **statistical** validation, and shift
   exp3's receipt from "cross-language hash-identity" to **"deterministic within Rust (same seed →
   same result) + statistically consistent with Python (similar emergence rates across seeds)."**
   Determinism-within-Rust is what gives the reproducibility receipt; cross-language bit-identity
   is a bonus gauss may forbid. Everything NOT downstream of gauss stays bit-exact.

## Port stages (each gated by a validation before the next)

**Status 2026-07: Stages 0–5 DONE.** exp3 anticipation emergence RAN. Honest result: the
`PROC_PREDICT` substrate **evolves unseeded (4/5 seeds)** but functional anticipation (negative
gaps) fires only **2/5** → **NOT confirmed** at the strict-emergence bar; refutes the unbacked
"89.2%" claim. Per-stage pieces bit-exact (VALIDATION.md); full result in
CONSTELLA_TO_EXPERIMENTS.md. **Next (v2): global waves crossing the full world + a MOVE op so
agents migrate → real non-local selection pressure for anticipation.**


- **Stage 0 — agent state + shield timing.** Add the fields above; record `last_shield_activation`
  when `ACT_SHIELD` fires in `execute_genome`. Gate: base sim still `a2bb005395f79766` (fields
  unused when no waves).
- **Stage 1 — WaveState + world init.** `WaveState` (start_tick, speed, active, stealth,
  direction; `front_position = elapsed*speed`; `is_complete` at `front>=GRID_WIDTH`), `spawn_wave`
  (**gauss — resolve the risk above**), `initialize_light_gradient` (deterministic center-out),
  `apply_thermal_drain` (`cell_light/255 * THERMAL_DRAIN_RATE`, ×0.3 for disruption phenotype).
  Gate: hash cell grid post-gradient (deterministic → must match Python); resolve gauss.
- **Stage 2 — wave damage.** `apply_wave_damage`: front-band `abs(agent.x-front)<1.0`, record
  `wave_arrival_times` (dedup >10 ticks, keep last 8), shield check (`genome[5|6]==ACT_SHIELD`),
  stealth = instant death. Gate: population/deaths trajectory vs Python.
- **Stage 3 — prediction + wave-aware sensing (memory stays OFF). ✅ DONE.**
  **CORRECTION to the original plan:** exp3 does NOT enable memory. `MEMORY_ENABLED=false` holds even
  in exp3 and gates *only* the memory op (agent.py:90 → MEM_NONE); `mem_pattern` is dormant. So we do
  NOT touch `memory_op → 0` (keeps base bit-exact AND matches exp3). Anticipation runs on
  `wave_arrival_times` (populated by wave *contact* in Stage 2) read by `proc_predict` (a *process*
  op, ungated). Ported + validated (`predicttest`, bit-exact vs Python; base hash intact):
  (a) `World.current_wave`; (b) wave-aware `sense_threat` = front proximity 0→255, stealth=0 (the
  reactive baseline: SENSE_THREAT→THRESHOLD→SHIELD); (c) `proc_predict` (process arm 4): extrapolate
  next arrival from history, fire within horizon = 3× SENSE_THREAT_RANGE. **Emergence signal is now
  crisp:** a reactive agent turns anticipatory by mutating P1 `PROC_THRESHOLD`(0)→`PROC_PREDICT`(4).
- **Stage 4 — detection + gap measurement.** `check_wave_detection` +
  `calculate_anticipation_gap`. Gate: gap distribution vs Python.
- **Stage 5 — harness + two arms.** exp3 loop (spawn on interval, process wave, thermal, tick),
  seeding for Arm A / Arm B, multi-seed sweep, fossil output of gap stats.

## Success criteria (what earns the word)

1. **Reproducible:** same seed → identical result in Rust (deterministic receipt). Non-negotiable.
2. **Strict emergence:** negative anticipation gaps appear in **Arm A (reactive-only)** across ≥3
   seeds. → `anticipation` CONFIRMED.
3. If (1) holds but (2) fails: honest tier stays `asserted`/refuted, documented as such. The port
   is a success either way — it replaces an unbacked claim with a receipt.

## RNG consumption order (transcribe per stage — do NOT guess)

The linchpin discipline from the base port: at each stage, read the Python method and transcribe
its exact `random.*` call order before writing Rust. Known so far: `spawn_wave` consumes
`gauss()` (→ up to 2 `random()` + transcendentals, with caching) then `random()` for stealth, in
that order, once per `PREDATOR_WAVE_INTERVAL`. All other exp3 additions (gradient, thermal, damage,
detection, gap) are **deterministic** (no RNG) — only memory-op mutation uses the existing
`mutate_genome` RNG already validated in the base.
