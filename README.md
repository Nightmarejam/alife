# ALife — a minimal ecology for studying emergence

**In one line:** strip the rules of a world down to almost nothing — energy, threat, a
gradient, decay — drop in agents with tiny genomes, and watch what evolves. The bet:
*simple rules generate seemingly infinite possibility.*

This sandbox is also a **governance testing zone**: it earns the confirmed vocabulary that the
[Constella framework](https://github.com/Nightmarejam/constella-framework) is built on. A civic
principle isn't trusted because someone asserted it — it's trusted because a reproducible
simulation produced it across many random seeds.

> Part of a personal AI ecosystem — see the [ecosystem map](https://github.com/Nightmarejam).
> Research-grade sandbox; results are intrinsically valuable independent of applications.
> Human-directed, AI-assisted, with receipts (confirmability tiers throughout).

---

## The idea (one concept)

A rich model of the universe would *confound* emergence — you couldn't tell whether complex
behavior came from evolution or from physics you baked in. So this world is deliberately
**minimal**: a grid where cells hold energy (food), threat (predator waves), light (a gradient),
and a slow thermal drain. A run "succeeds" when it produces **something the experimenter did not
predict from the rules.**

---

## How the engine works

There is no hidden brain. This whole section describes ~2,000 lines of Rust (with a Python twin
for every file), and it fits in your head.

### An agent *is* its genome — eight bytes, read like a tiny program

Each organism carries an 8-byte genome. Every byte is an instruction slot, grouped into five
jobs. To run an agent for one tick, the engine reads its slots in a fixed order and does what
they say. Evolution is just these bytes being copied — with rare mutations — on reproduction.

| bytes | job | what it does |
|---|---|---|
| 0–1 | **sense** | read a number from the world or itself |
| 2–3 | **process** | turn a reading into a yes/no |
| 4 | **memory** | keep past readings *(dormant in the base sim — `MEMORY_ENABLED=false`)* |
| 5–6 | **act** | change the world (only if process said yes) |
| 7 | **regulate** | manage the agent's own energy rules |

Each slot only reads its lowest 3 bits (`byte & 7`), a number 0–7 — so every slot picks one op
from its category's menu of eight. That menu lives in `ops.rs` and is the entire behavioral
repertoire of the sandbox:

- **sense:** energy · threat · light · neighbor · density · self · gradient · age
- **process:** threshold · compare · memory-cmp · trend · predict · beat · average · invert
- **memory:** none · last-1 · last-4 · last-8 · best · worst · pattern · dual
- **act:** idle · move · consume · shield · reproduce · signal · toxin · flee
- **regulate:** none · conserve · burst · cycle · learn · suppress · prioritize · adaptive

### The engine is a short stack — each layer rests on the one below

Read it bottom-up; nothing ever calls upward.

| file (Rust) | role | in plain language |
|---|---|---|
| `rng.rs` | the bedrock | a bit-for-bit rebuild of Python's random generator — the source of all randomness |
| `world.rs` | the ecology | the 480×360 grid of cells, energy sources, shocks, seasons, and the *state hash* |
| `agent.rs` | the organism | one creature: genome, energy, position, age, and the *population hash* |
| `ops.rs` | the repertoire | the 40 ops (5 menus above) — everything an agent *can* do |
| `sim.rs` | the clock | the tick loop: drain, run genomes, births and deaths, the experiment knobs |
| `main.rs` | the front desk | picks an experiment, sets the knobs, runs the ticks, writes a fossil receipt |

The Python originals (`world.py`, `agent.py`, `ops.py`, `simulation.py`) are the readable
reference; the Rust core is the same simulation rebuilt for speed.

### One tick, in order

The loop in `sim.rs`, in plain language — the same sequence repeats thousands of times per run:

1. **Refill the world.** The clock advances; every fifth tick, energy sources top up nearby cells.
2. **Maybe shock the map.** If this experiment uses shocks, energy sources jump to new spots (and
   sometimes a famine hits) — the moment that tests whether a diverse population can adapt.
3. **Shuffle the turn order.** Agents act in a randomized order each tick, so nobody keeps a
   permanent first-mover advantage.
4. **Run each agent's genome.** Pay the cost of existing, then read the 8 bytes in order —
   **regulate → sense → process → act.** Actions fire only when process returned "yes." An agent
   that hits zero energy dies here (unless a floor rescues it).
5. **Handle reproduction.** Eligible agents spawn a child nearby, its genome copied with rare
   mutations. Density and the accountability cap make crowding and dominance cost more.
6. **Clear the dead.** Dead agents leave the grid, freeing their cells. Then the next tick begins.

### How a run becomes a confirmed word

A behavior isn't trusted because it happened once — it's trusted because it happens *every time*,
across many seeds. That's confirmability applied to emergence:

```
many seeds  →  fossils (JSON receipts)  →  reproducible across seeds?
     →  yes: confirmed   varies: speculative   never: honest negative
     →  the word crosses the evidence bridge into Constella
```

`pipeline_extract.py` does the "reproducible across seeds?" test; the bridge is
`alife_evidence_mapping.md` in the Constella repo.

### The determinism spine

The sandbox is only trustworthy if Python and Rust produce the *identical* result — so both are
pinned to the same two fingerprints, re-verified after every change to the engine:

| fingerprint | value (seed 42) | check |
|---|---|---|
| sim state @ 100 ticks | `a2bb005395f79766` | `alife-core run 42 100` |
| population @ init | `2f59d3550af7cf2f` | `alife-core pop` |

Because `rng.rs` reproduces Python's generator bit-for-bit and the hashes use plain FNV-1a
arithmetic, the two engines stay locked together. That lock — not the speed — is the real asset.

---

## What's here

- `simulation.py`, `world.py`, `agent.py`, `ops.py` — the Python sim (the readable reference).
- `alife-core/` — the **Rust port** (fast + bit-exact). Run `alife-core help` for the full
  catalog of governance experiments and determinism probes.
- `experiments/` — exp0 primordial → exp5 parasitic (Red Queen) → exp6–9 (cultural / diversity).
- `fossil_run.py` / `fossil_compare.py` — seeded runs → hashed local fossil records
  (cross-location integrity checks; no database needed).
- **Docs (read these for the thinking):**
  - `CONSTELLA_TO_EXPERIMENTS.md` — the concept → experiment → vocabulary ledger.
  - `WORD_SCHEMA.md` — the shape of a confirmed word (claim / holds-when / fails-when / receipt).
  - `SYNTHESIS.md` — the predictability law and its crux tests.
  - `SANDBOX_REVIEW.md` · `WORLD_DESIGN_LINEAGE.md` · `NOMENCLATURE_PIPELINE.md` — audit and lineage.

## How it connects to FAITHH

Loose coupling by design: **ALife exports a validated vocabulary dataset**, and **FAITHH imports
it** into its knowledge base — always carrying the tiers (concept confirmed in-sim; any human
mapping stays speculative). ALife doesn't live inside FAITHH; it produces a knowledge artifact
FAITHH reads.

## Status (honest, tiered)

- **Sandbox & design:** ✅ hold up — sound minimal-emergence complexity science.
- **Rust port:** ✅ RNG + World + Agent + the **full tick loop** validated bit-for-bit vs Python
  (anchors above). The port is complete and is the canonical engine for scaled runs.
- **Results:** ~16 **confirmed** words earned across seeds (with boundaries), plus honest
  negatives where a claim did not reproduce — the discipline held, no faked receipts. See
  `CONSTELLA_TO_EXPERIMENTS.md`.

## Roadmap

1. ~~Finish the Rust tick loop~~ ✅ done — validated by multi-tick state hash.
2. Continue earning words: run seeded experiments at scale, banking confirmed / honest-negative.
3. Nomenclature pipeline → the exported dataset → FAITHH ingestion (gated on the rig).
4. Future track: convergence of "cultural universals" across independent populations.
