# Validation receipts (alife-core)

Every determinism claim, its golden value, and the command that reproduces it. The bar: Rust ==
Python, bit-for-bit, seed 42. If a value here stops reproducing, something regressed — treat it as
a failing test. (Built from `alife-core/`; run `cargo build --release` first.)

## Base sim — cross-language bit-exact (seed 42)

| What | Golden value | Reproduce | Cross-lang |
|---|---|---|---|
| RNG (`random()`, `randrange`) | asserts pass | `cargo run --release` | ✅ vs `python3 -c "random.seed(42)…"` |
| World grid after init | `76aae69c71538657` | `alife-core world` | ✅ both langs |
| Population (50 agents) | `2f59d3550af7cf2f` | `alife-core pop` | ✅ both langs (re-confirmed 2026-07) |
| Sim state @ 100 ticks | `a2bb005395f79766` | `alife-core run 42 100` | ✅ both langs |
| Fossil @ 100 ticks | `a2bb005395f79766` | `alife-core run 42 100 fossil` | ✅ both langs |

The `a2bb005395f79766` state hash is the **primary gate**: any change to the core must leave it
unchanged (it covers population + world + 100 ticks of full dynamics). It held through the UCF
knobs and the exp3 f64-energy refactor.

## exp3 port — per-stage receipts (see EXP3_PORT_SPEC.md)

| Stage | What | Golden / check | Reproduce | Cross-lang |
|---|---|---|---|---|
| 0 | agent anticipation state + shield hook | base hash unchanged | `alife-core run 42 100` | ✅ (inert without waves) |
| 1 | `gauss` (transcendental RNG) | 6/6 f64 bits identical | `alife-core gausstest` | ✅ vs Python `random.gauss` |
| 1 | `spawn_wave` (speed+stealth) | 6/6 identical | `alife-core wavetest` | ✅ vs Python `spawn_wave` |
| 1 | `initialize_light_gradient` | `b295c06633458b3c` | `alife-core gradienttest` | ✅ both langs |
| 1 | f64 energy refactor | base hashes unchanged | `run 42 100` + `pop` | ✅ behavior-preserving |
| 1 | `apply_thermal_drain` | formula-identical (0.2 @ light 255) | (integrated at Stage 5) | pending Stage 5 |
| 2 | `apply_wave_damage` | damage/shield/death/arrival logic (kills=2, shielded=1) | `alife-core wavedamagetest` | ✅ deterministic; trajectory at Stage 5 |
| 3 | `sense_threat` (wave proximity) + `proc_predict` (anticipation) | proximity 0/51/102/255, stealth 0; predict fires only in horizon | `alife-core predicttest` | ✅ vs Python formulas (memory stays OFF — see spec) |
| 4 | `check_wave_detection` (the anticipation-gap instrument) | gap=−5 anticipatory / 0 / none / excluded; neg_gaps=1 | `alife-core gaptest` | ✅ vs Python |
| 5 | two-arm emergence harness (reactive-only vs seeded) | — | — | DONE: Arm B confirms instrument (1906 gaps); Arm A reactive-only = PROC_PREDICT evolves 4/5, fires 2/5 -> NOT confirmed (honest) |

## Note on platform
The transcendental bit-exactness (Stage 1 gauss) holds on the Mac dev machine (shared libm).
A different libm (e.g. Linux/Gen8) could diverge on `cos/sin/ln`; if so, the wave-speed layer
falls back to deterministic-in-Rust + statistical-vs-Python (see EXP3_PORT_SPEC.md). Everything
else is integer/fixed and cross-platform reproducible.
