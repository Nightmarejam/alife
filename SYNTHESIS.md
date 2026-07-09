# Synthesis — what the ALife experiments say together
*2026-07-08. Tier: **asserted** (a pattern across confirmed results, not itself yet confirmed). One
load-bearing claim is untested — flagged below and tested in this same doc.*

The individual results (`civic-floor`, its seasons boundary, `anticipation`, `metabolic-thrift`,
buffering, convergence-under-cycles) are not independent. They circle a single axis.

## The law (asserted)
> **The predictability of environmental change governs the optimal adaptive strategy — and the value
> of every intervention.**
>
> - **Predictable pressure** (constant *or* cyclical) → the system **CONVERGES** on one cheap, robust,
>   anticipatory strategy. Diversity is overhead.
> - **Unpredictable pressure** (abrupt shifts that open a genuine new niche) → the system must hold
>   **DIVERSITY in reserve**, and a **floor** to preserve that reserve becomes load-bearing.
>
> Corollary (**floor–volatility coupling**): *the value of a diversity-preserving floor is proportional
> to the environment's unpredictability.* Predictable world → reserve is dead weight → floor is overhead
> (can even feed overshoot). Unpredictable world → floor is insurance that pays.

## What it rests on (confirmed / earned results)
| result | which side of the axis | fits the law as |
|---|---|---|
| `anticipation` (global/regular waves 5/5; local/irregular refuted) | predictable → converge | predictable pressure lets foresight evolve; unpredictable pressure starves it |
| `metabolic-thrift` (seasons Step 1) | predictable-cyclical → converge | a predictable cycle → converge on the cheapest all-weather regulator |
| buffering (seasons Step 2) | predictable-cyclical → converge | population absorbs a predictable cycle internally; no diversity needed |
| convergence-under-cycles (seasons Step 4) | predictable-cyclical → converge | even a *direction-flipping* predictable cycle converges |
| `civic-floor` + `diversity-reserve` (dmech) | unpredictable-shift → diversify | abrupt regime flip **+ new niche** → floor-preserved reserve becomes the winner |
| civic-floor boundary (seasons Step 3) | predictable-cyclical → floor overhead | under a predictable cycle the floor is marginal-to-harmful (overshoot) |

Every row is the *same finding* in different clothes: **predictable ⇒ converge (floor is overhead);
unpredictable ⇒ diversify (floor pays).**

## The untested crux
**Predictability has never been the manipulated variable.** Every experiment *fixed* a regime
(constant / periodic / global) and read the outcome. None ever **swept predictability** with everything
else held equal. So the load-bearing claim of the whole theory is the one thing not yet tested.

**The test (this doc, below):** identical world, identical mean rate of environmental change — vary only
whether the change is **periodic (predictable)** or **random-timing (unpredictable)**. Prediction:
- periodic → converge (low diversity), floor adds little;
- random → diversity maintained / needed, and the **floor helps more** (floor–volatility coupling).

If the converge↔diversify flip *and* the floor-value flip track the predictability knob, the law earns
**word #3 as a principle** (candidate: `predictability-tunes-strategy` / `volatility-floor-coupling`).
If they don't, the law is bounded or refuted — an honest result either way.

## Crux results (2026-07-08) — the spine holds; the specific crux is REFINED, not confirmed
Setup: `amp 0` (pure directional flip, no thermal), `period 800` (mean flip interval 400), split on the
sense locus, **14 seeds**. Periodic (`FLIP`) vs random-timing (`RANDFLIP`), **matched mean rate**;
±`FLOOR=30`; penalty `dirp ∈ {1,2,3}`. Survivors / 14:

| penalty | FLIP no-floor | FLIP +floor | RAND no-floor | RAND +floor |
|---|---|---|---|---|
| dirp 1 (mild)   | 14 | 14 | 14 | 14 |
| dirp 2 (moderate) | 10 | **14** |  8 | **14** |
| dirp 3 (harsh)  |  4 | **14** | 10 | **14** |

**1. Floor is decisively load-bearing under directional flips. ✅ (spine confirmed)** Every floored arm
is **14/14**, vs 4–10/14 without. This reaffirms `civic-floor` and *extends* it to cyclical directional
shifts — a clean win under *shifting* pressure (flips), just as under the confirmed `dmech` shocks, and
in sharp contrast to the seasons *intensity* cycle where the floor was marginal/harmful (Step 3). The
law's spine — **shifting/unpredictable pressure → the floor pays** — holds.

**2. Floor–volatility coupling: NOT cleanly testable here.** The floor saturates survival at 14/14 in
every arm (ceiling effect), so any periodic-vs-random *differential* in floor value is masked. The
corollary is neither confirmed nor refuted by this design — it needs a regime where the floor doesn't
max out.

**3. Predictability's effect is NON-MONOTONIC — the honest surprise.** No-floor survival, periodic vs
random, *reverses sign* with intensity:
- dirp 2 (moderate): FLIP 10 **>** RAND 8 → unpredictability **hurts**.
- dirp 3 (harsh):    FLIP 4 **<** RAND 10 → unpredictability **helps**.

*Mechanism — "metronome vs reprieve":* a **predictable** penalty is a relentless metronome — every
cycle it bleeds the disfavored group for exactly `half` ticks with no relief; at harsh intensity that
grind is simply lethal. **Random** timing is high-variance: occasional *short* stretches give recovery
windows (help), occasional *long* stretches are extra-lethal (hurt). Which dominates depends on
intensity — mild → the long stretches are the dominant risk (random worse); harsh → the relief windows
are the lifeline (random better).

### Verdict
The naive law — *"unpredictable → worse → need a floor"* — is **REFINED, not confirmed.** Unpredictability
cuts *both ways* (variance = both longer-bad and shorter-bad stretches), so predictability does **not**
monotonically tune survival. No clean **word #3**.

**Honest artifacts:**
- **REAFFIRMED:** `civic-floor` is load-bearing under directional flips — periodic *and* random —
  extending the confirmed result from abrupt shocks to cyclical shifts.
- **NEW candidate refinement** (needs its own confirmation): **`metronome-vs-reprieve`** — a *predictable
  relentless* stressor can be **deadlier** than a *random* one of equal mean intensity; unpredictability's
  sign flips with stressor intensity. This is a genuinely non-obvious result and a better prize than the
  monotonic law would have been.
- **CAVEAT (bounds the whole test):** no truly symmetric group locus exists (genome is 3-bit — no free
  neutral tag), so this measured **survival / floor-value**, not a clean **diversity-maintenance** signal.
  The pure *diversify-vs-converge* half of the law remains **untested**; the interface→diversity or a
  neutral-tag redesign would be needed to reach it.

**Re-tier:** law *spine* (shifting → floor pays) = **supported** (now shown for cyclical flips too);
*specific crux* (monotonic predictability tuning / floor-volatility coupling) = **refuted for survival**,
replaced by the non-monotonic `metronome-vs-reprieve` refinement.

## B1 (2026-07-09) — `metronome-vs-reprieve` REFUTED as general; the effect is STRUCTURE-dependent
Confirmation experiment (`pulse` mode): a **single global ON/OFF stressor** (drain `L` when on, no
groups, no waves), matched duty (0.5) + mean bout length (100), varying **only the timing variance** —
periodic (fixed) / inter (uniform ±50%) / random (geometric). 14 seeds, survivors:

| L | periodic | inter | random |
|---|---|---|---|
| 3.5 | 11 | 6 | 5 |
| 4.0 | 10 | 6 | 1 |
| 4.5 | 1 | 1 | 1 |
| 5–6 | ~1 | 0 | ~0 |

**The opposite of the crux, and monotonic.** For a *global* stressor, survival is `periodic > inter >
random` across the entire survivable band — **more timing-variance is monotonically *worse*, with no
harsh-side crossover.** Mechanism: a global stressor hits everyone at once, so an unusually long
(geometric-tail) bout wipes the population **synchronously** — and unlike the directional flip, there is
**no differentially-exposed reservoir** left to exploit a later reprieve.

**Verdict — `metronome-vs-reprieve` REFUTED as a general claim:** its sign is not universal, it *reverses*
with population structure. What the two experiments jointly establish:
- **Uniform/global stress → regularity is SAFER; unpredictability monotonically hurts** (pulse, 14 seeds,
  clean, no caveats). *This* is the reliable finding.
- Differential/rotating stress → unpredictability *can* help at harsh intensity (flip crux — but carries
  the group-asymmetry caveat).

**The real principle (asserted, rests on both regimes): the value of unpredictability is
STRUCTURE-DEPENDENT.** Variance helps only when a *differentially-exposed reservoir* survives each bout
to exploit the reprieve; without structure (uniform stress) unpredictability just adds lethal long-tail
bouts. **Population structure is the precondition for variance to ever help.**

**Re-tier:** `metronome-vs-reprieve` → **refuted (general); bounded to differential stress.** New clean
sub-finding: *regularity buffers uniform stress* (candidate — 14-seed monotonic, well-supported). New
asserted principle: *unpredictability's value is structure-dependent* (rests on pulse + flip together).
