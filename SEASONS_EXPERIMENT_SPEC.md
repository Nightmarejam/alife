# Seasons experiment — design spec

*2026-07. Origin (Jonathan): "what were our ancestors' first concerns when life was still forming?
Weather and seasons, the balance of the universe." Cyclical environmental adaptation — dormancy,
hoarding, circannual timing, migration — is a near-primordial evolutionary problem, older than
predators. This experiment tests how a population learns to ride the cycle.*

## Why it is NOT the wave (the whole point)
| | Wave (exp3) | Season (this) |
|---|---|---|
| shape | spatial, discrete, localized, **instant-lethal** | global, gradual, whole-world, **sub-lethal drain** |
| survive by | shield / flee / predict the front | **build a reserve, downregulate metabolism, time reproduction, migrate** |
| adaptation | acute/reflexive | chronic/strategic |
Different toolkit → not a redo. It probes a distinct class of adaptation.

## The mechanic (cheap — the thermal machinery already exists)
Today `apply_thermal_drain = (cell_light/255) * THERMAL_DRAIN_RATE` (rate 0.2, constant).
**Season = oscillate the rate over a cycle:** `rate(t) = 0.2 * season_factor(t)`, where
`season_factor` swings between `1-amp` (summer, low drain) and `1+amp` (winter, high drain) over
`period` ticks. Use a **triangle wave** (piecewise-linear ramp up/down) — no transcendental, fully
deterministic, and a clean "ideal → harsh → ideal" transition. (sin would also work — gauss proved
transcendentals are bit-exact here — but triangle is simpler.) Defaults to sketch: amp≈0.8,
period≈2000 → winter drains ~9× summer. Add `season_amp`, `season_period` as sim params; the
harness (or apply_thermal_drain) applies the factor. Constant-rate (amp=0) = today's behavior →
base hash unaffected.

## Adaptations it could select (all already expressible in the genome)
- **Reserve / hoarding** — accumulate energy toward the cap (255) in summer to survive winter.
  (Energy already banks; reproduction-timing gates it.)
- **Metabolic downregulation / torpor** — the REGULATE ops already apply a `cost_modifier`; ops
  that cut costs when energy is low = evolved torpor. Watch the regulate-op distribution.
- **Phenological timing** — reproduce when energy is high → naturally concentrates in summer.
  Emergent phenology; measure repro-timing vs season phase.
- **Migration** — MOVE toward milder cells (if a spatial thermal gradient exists; the light
  gradient already makes edges cooler → edges = winter refuge).

## Arms / conditions
1. **Baseline seasonal (no floor).** Does the population survive seasonal cycling? What evolves —
   regulation, timing, reserves? Multi-seed. (Does it boom-bust with the cycle, or stabilize?)
2. **★ Floor vs no-floor under seasons (the civic test).** Does an energy floor (rescue in winter)
   raise survival? This is the most policy-relevant run we can do — *"build reserves in the good
   time to survive the lean time" IS the floor.* Directly tests the topsoil/agriculture theme.
3. **Diversity vs convergence under cycles (completes the V3 story).** V3 showed reaction-diversity
   is FATAL under a *constant* lethal threat. Seasons are the *cyclical/shifting* case where the
   civic-floor result predicts diversity HELPS (a reserve of strategies for when conditions swing).
   Run diverse-strategy vs uniform under seasons → does diversity now win? If yes, the diversify-vs-
   converge principle is nailed empirically (converge under constant threat, diversify under cycles).
4. **(v2, advanced) Anticipatory seasons.** For the *cognitive* test, agents need to sense the
   seasonal phase + predict the downturn (analogous to wave_arrival_times, but for the cycle — a
   "season clock"). Then: does foresight-of-the-cycle emerge (prepare BEFORE winter)? Extends the
   confirmed `anticipation` word to *strategic* (vs reflexive) anticipation.

## Readouts
Population trajectory vs season phase (boom-bust? stable?); regulate-op distribution (torpor
selected?); mean energy just before winter (hoarding?); reproduction timing vs phase (phenology?);
extinction rate across seeds; floor arm: survival + winter-trough depth (floor vs no-floor).

## Candidate words (tiers earned only by multi-seed reproduction)
*seasonal-reserve / hoarding*, *metabolic-downregulation (torpor)*, *phenological-timing*,
*seasonal-floor* (floor→survival under cycles), *diversify-under-cycles* (the complement to V3).

## Connection to the whole project
- Completes the **diversify-vs-converge** principle (V3 = constant→converge; seasons = cyclical→
  diversify).
- The **floor under seasons** is the sharpest civic-floor test — reserves-for-lean-times is the floor.
- Extends **anticipation** from reflexive (wave) to strategic (prepare-for-winter).
- Grounds Jonathan's **topsoil/boom-bust** concern in a runnable model.

## Success / graduation
A candidate word graduates when its adaptation appears **reproducibly across ≥3 seeds** with a
deterministic receipt (same bar as the floor + anticipation). Honest negatives count: if seasons
just trivially track (no interesting strategy), that's a real result too.

## Build order (low → high cost)
1. Oscillating thermal (triangle factor) + base-hash check (amp=0 unchanged). ✅ **DONE** (see below)
2. Baseline seasonal run + readouts (regulate dist, energy-before-winter, repro-timing). ✅ **DONE**
3. Floor vs no-floor seasonal arm.
4. Diversity-vs-convergence arm.
5. (v2) season-clock sensing + anticipatory-seasons.

---

## Step 1 results (2026-07, `seasons` mode) — mechanic built + first observations
Mechanic: `world.seasonal_drain()` = `season_amp * (1 − |2·phase − 1|)` (triangle, 0 at midsummer →
`amp` at deep winter) added into `apply_thermal_drain`; `amp=0` ⇒ untouched (base `run` hash still
`a2bb005395f79766`, pop `2f59d3550af7cf2f`). Run mode `seasons <seed> <ticks>` (env `AMP`, `PERIOD`),
no waves, 8-bucket phase profile over the final 4 years. Founding pop 100, period 2000.

**Finding 1 — a survival phase-transition (~amp 1.2–1.5).** Winter amplitude vs survival across 6
seeds {42,1,7,123,999,2026}:
| amp | deep-winter net/tick | outcome across 6 seeds |
|----|----|----|
| 1   | ≈ +3 (still positive) | **6/6 survive** (final 478–890), stable |
| 1.5 | ≈ +2.5 | knife-edge: 2 healthy, 2 collapse-to-1, 1 extinct, 1 ok |
| 2   | ≈ +2 | mostly collapse: 2 healthy, 2 →1, 2 extinct |
| 3–5 | ≤ +2 w/ depletion | collapse / extinction |
The population rides a *gentle* cycle indefinitely but a harsh winter tips it past a threshold into
runaway collapse (often to a single survivor or extinction). The threshold is sharp and seed-sensitive
right at the edge — a genuine **resilience limit**, and it hands Step 3 its exact test regime.

**Finding 2 — robust metabolic-thrift selection (NOT active torpor).** Where populations survive, the
winning REGULATE op is *always* from the cheap set — `low-cut`(1, the real −1 cost-cutter), `crit-cut`(5),
or `none`(0) — never a costly always-pay regulator. `REGULATE_COSTS=[0,0,1,1,1,0,1,2]`: ops 3/4/6 burn
1/tick and op 7 (`adaptive`) burns 2/tick *every tick*. Selection purges them under the chronic winter
squeeze. **Counterintuitive core result:** the intuitive "torpor" op (7 `adaptive`, cuts −1 when low)
gets *eliminated* because its 2/tick base cost outweighs its discount — cheap **inaction/thrift** beats
costly **active downregulation**. At amp 1 the genuine cost-cutter `low-cut` co-dominates with the
zero-cost ops (seed 42: low-cut 465 / crit-cut 417; seed 999: low-cut 874). *Which* zero-cost op wins is
drift; the *cheapness* is selected. Refines the candidate word: **torpor only pays if the regulation
itself is nearly free.**

**Finding 3 — cycle shows in energy, not (yet) in headcount.** At the survivable amp the population
pins to carrying capacity (~880, flat across phases); the season reads as a mild mean-energy dip in deep
winter (~250→235, ≈6%). A *population*-level boom-bust only appears at the collapse-prone amplitudes.
→ Tension for Step 2/3: to see a headcount cycle without courting extinction, either lower the carrying
cap (so winter culls show) or run at the knife-edge amp *with the floor* — which is precisely the
Step-3 hypothesis: **does an energy floor convert chaotic-collapse (amp 1.5–2) into robust survival?**
That is now the sharpest civic-floor test we can run, with the amplitude pre-located.

---

## Step 2 results (2026-07) — baseline polish: the population BUFFERS the season
Added carrying-capacity knobs (`density_onset` via `CAP`, `density_div` via `DIV`; both default to the
base 150/5 → other modes bit-identical) to try to expose a *headcount* cycle, plus a cycle-summary
readout (energy swing, headcount swing, repro-timing ratio).

**Core finding — buffering, not tracking.** At *every* survivable parameter regime the steady-state
headcount is **flat** (density-limited: any winter gap refills instantly), and the season instead shows
up **internally**:
- **Energy cycle** — tunable and clean. amp 1 → 6% swing (250→235); **amp 4 / period 300 → 49% swing
  (233→119)**, a near-2× sinusoidal trace across the year. This is the "visible cycle," just in energy,
  not bodies.
- **Metabolic-thrift selection** — robust (from Step 1): costly always-pay regulators purged; survivors
  carry cheap regulators (crit-cut/low-cut/none).
- **Repro timing is NOT phenology (honest negative).** At carrying capacity, births are winter
  *replacement* (they track deaths opening slots), so they cluster in the cold half — the opposite of
  adaptive summer breeding. Confounded by the density cap; a clean phenology test needs a below-capacity
  regime or a repro-*request* (intent) metric. Not claimed.

**No stable headcount cycle exists — the system is bistable.** Buffered-flat (survives) OR runaway
collapse (extinct). Large headcount swings appear only in the startup transient or right at the collapse
edge — there is no stable large-amplitude population cycle in between. Trying to force one by cranking
amplitude just trips the collapse. (This matches real seasonal species: stable numbers via stored
reserves + birth timing, not an annual crash.)

**Knife-edge regime located for the floor arm.** `amp 4 / period 300`, default cap, is a clean 50/50
across 6 seeds: **survive {42, 123, 2026}, extinct {1, 7, 999}**. That is the floor arm's test bed —
*does an energy floor convert the 3 collapse seeds into survivors (→ 6/6), and does it do so without
killing the metabolic-thrift signal?* (Slow-season alt: `amp 2 / period 2000`, also knife-edge.)

Run: `AMP=4 PERIOD=300 alife-core seasons <seed> 20000`.
