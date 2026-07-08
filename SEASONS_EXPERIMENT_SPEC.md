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
1. Oscillating thermal (triangle factor) + base-hash check (amp=0 unchanged). 
2. Baseline seasonal run + readouts (regulate dist, energy-before-winter, repro-timing).
3. Floor vs no-floor seasonal arm.
4. Diversity-vs-convergence arm.
5. (v2) season-clock sensing + anticipatory-seasons.
