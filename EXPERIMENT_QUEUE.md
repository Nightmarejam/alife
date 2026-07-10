# ALife experiment queue

*2026-07. What we can actually run in the sandbox (→ confirmed words for the FAITHH/Constella
nomenclature), vs. what's banked for a proper lab. Focus here; bank the rest.*

## 📍 RESUME POINT — stable state as of 2026-07-08
*Read this first to continue cold. Everything below is committed; the tree is clean.*

**Determinism anchor (never breaks):** base `run` `state_hash a2bb005395f79766`, `pop`
`population_hash 2f59d3550af7cf2f`. Every experiment knob (seasons amp/period/cap/div, flip, floor,
dir_locus) defaults to base-identical — verify these two hashes after any core change.

**Words banked so far:**
- **#1 `civic-floor`** — CONFIRMED (floor→resilience; dignity-vs-diversity split; targeted floor fires
  the reserve). **+ NEW boundary (seasons Step 3):** the floor is *context-dependent* — marginal-to-
  harmful under cyclical drain (feeds overshoot); helps only vs adversity that *removes variation*.
- **#2 `anticipation`** — CONFIRMED conditionally (5/5 under GLOBAL waves; refuted under local waves;
  episodic). See `CONSTELLA_TO_EXPERIMENTS.md`.
- **`metabolic-thrift`** — NEW refinement (seasons Step 1): cyclical stress selects *cheap* regulation;
  the active-"torpor" op is purged for costing more than it saves.
- **`metronome-vs-reprieve`** — **REFUTED as general** (B1 pulse test, `SYNTHESIS.md`): under a *global*
  stressor, timing-variance monotonically HURTS (periodic 11 > inter 6 > random 5 /14) — the effect's
  sign *reverses* with population structure. Bounded to *differential* stress only.
- **`unpredictability is structure-dependent`** — NEW asserted principle (B1 + flip crux): variance helps
  only when a differentially-exposed *reservoir* survives each bout; uniform stress → variance only hurts.
  Sub-finding *regularity-buffers-uniform-stress* is clean (14-seed monotonic).
- **`targeted-floor` → stable-adaptive regime** — CONFIRMED (B2, Rust 10-seed, reproduces Python Exp 9):
  a floor preserving only MINORITY strategies keeps an adaptive adversary from specializing (adapt LOW,
  survives 8/10). Reconfirms `unconditional-floor → stasis` (adapt maxes 1.50 in 10/10 — life-support)
  and `no-floor → gamble` (extinct 3/10). First entry in the WORD SCHEMA (see CONSTELLA_TO_EXPERIMENTS.md).
- **`adaptation-speed-limit`** — CONFIRMED (C1, Rust 12-seed ratio grid, reproduces Exp 5): collapse under
  an adaptive adversary is set by the adversary/agent speed **ratio**, not absolute resources — the
  survival threshold `RISE` scales with agent mutation rate. Feeds Astris decay-rate. See ledger.
- **`entrainment`** — CONFIRMED (B3, Rust 8-seed): an endogenous clock locks to an exogenous periodic
  rhythm and TRACKS it (100% period-lock, 15/16 at P=40/60). Phase alignment is nurture-dominant
  (calibration 0.64→0.97) + nature adds on top (heritable phase →1.02) = *always both, in a ratio*.
  Boundary: fails when the rhythm is optional (long period/gentle economy → no selection). Grounds the
  nature/nurture framing; extends `anticipation` (reflexive → rhythmic). See CONSTELLA_TO_EXPERIMENTS.md.
- **`founding-diversity` (load-bearing)** — CONFIRMED (B4, Rust 10-seed, reproduces Python Exp 8b):
  diverse-founded + targeted floor survives 8/10 (adapt low); mono-founded + same floor only 5/10 (adapt
  high — predator specializes despite the floor). Diversity must be present at founding; a floor
  preserves a reserve but can't reliably create one late. Boundary: occasionally bootstraps (not strictly
  required). Pairs with `targeted-floor` (floor = maintenance, founding diversity = the substrate).
- **The predictability law** (`SYNTHESIS.md`, tier *asserted*): spine **supported** (shifting pressure →
  floor pays, now shown for cyclical flips too — floor took survival to 14/14 in every arm); the specific
  *monotonic* crux **refuted** for survival (→ metronome-vs-reprieve). Diversity half still **untested**
  (no symmetric genome locus available — genome is 3-bit).

**Experiments COMPLETE (closed, documented):**
- **exp3 anticipation** (V1 local / V2 global / V3 open reactions) → `CONSTELLA_TO_EXPERIMENTS.md`.
- **Seasons** (Steps 1–4) → `SEASONS_EXPERIMENT_SPEC.md`. Net finding: **convergence is the attractor
  under *predictable* cyclical pressure** (both intensity- and direction-flip cycles converge). Open
  hypothesis it raised: *predictability/niche-structure* is the governing variable for diversify-vs-
  converge (untested).

**The ONE clean unbuilt experiment (best resume target):** *interface → diversity.* Add wet/dry
**zones** (a persistent spatial boundary that sweeps with the tide) to the Step-4 flip, and test
whether a persistent interface *rescues the diversity the uniform flip destroyed* — closing Step 4's
negative. Design + honest scope in `research-notes/bio-acoustics/interface-molecules-and-the-cycle-as-
organizer.md` (the deeper amphiphile/self-assembly part is banked as beyond-tool, do NOT build it).

**Everything speculative → `research-notes/`** (the holding pen), tier-tagged. Do not let it leak into
confirmed reasoning.

**To continue:** Batch B1–B4 ✅ COMPLETE (metronome→structure-dependence, targeted-floor, entrainment,
founding-diversity). Both Constella word-sync PRs (#62, #63) merged. **Next: BATCH C1–C4** (see below) —
start with **C1 `adaptation-speed-limit`** (cheapest; a pure `RISE` sweep on the built b2 adversary).

---

## ★ NEXT (teed up): confirm `metronome-vs-reprieve`
*From the crux run (`SYNTHESIS.md`): a predictable *relentless* stressor was deadlier than a random one
of equal mean intensity, and the sign flipped with intensity. Candidate word — needs its own clean
confirmation, on a design that removes the crux run's caveats.*

- **Claim:** for a stressor harsh enough that a *continuous bout* is lethal but *intermittent* exposure
  is survivable, **higher-variance (random) timing yields higher survival than periodic timing at equal
  mean intensity** — and this **reverses at mild intensity**. I.e. predictability's survival effect is
  **non-monotonic in stressor intensity** (mild → random worse; harsh → random better).
- **Clean design (kills the caveats):** drop the directional groups entirely — test a **single global
  pulsed stressor** (a "drought/frost" pulse: thermal drain = `L` when ON, 0 when OFF), fixed **duty
  cycle**, no genome-group asymmetry. Two timing regimes at matched duty + mean bout length:
  **PERIODIC** (regular on/off) vs **RANDOM** (geometric on/off), plus one **intermediate-variance**
  arm to show monotonicity in variance. Sweep intensity `L` × timing, ≥12 seeds.
- **Receipt (what confirms it):** `survival(random) − survival(periodic)` **crosses zero** as `L` rises
  (negative at mild `L`, positive at harsh `L`), reproducibly across seeds. Bonus: survival rises
  monotonically with interval-variance at harsh `L`.
- **Null / refutation:** no sign-flip (random uniformly ≥ or ≤ periodic) → it's a plain variance effect,
  not the intensity-dependent `metronome-vs-reprieve` mechanism → candidate refuted.
- **Build cost:** small — add a pulsed-drain mode to `seasons` (drain `L` when on; periodic vs random
  on/off via the existing `flip_rng`). No new core mechanics; base hash stays `a2bb005395f79766`.
- **If confirmed:** first clean **word #3** since `anticipation` — and a non-obvious one (regularity of
  adversity, not just its amount, sets lethality).

---

## 🎯 NEXT BATCH (toward the 20-confirmed-words goal) — set up 2026-07-08
*A coherent set aimed at earning confirmed words that map to Constella mechanics (so each success
flows straight into the evidence bridge). Run in this order — cost rises, and B2/B4 share the
threshold-floor build. Each lists: hypothesis · candidate word · Constella mechanic it feeds · receipt.*

**B1 — `metronome-vs-reprieve` confirmation.** ✅ **DONE (refuted-but-bounded).** Global pulse, 14 seeds:
timing-variance monotonically *hurt* survival (periodic 11 > inter 6 > random 5) — no harsh-side
crossover. The effect is **structure-dependent** (needs a differentially-exposed reservoir); refuted for
uniform stress. Yielded the asserted principle *unpredictability's value is structure-dependent* +
clean sub-finding *regularity-buffers-uniform-stress*. See `SYNTHESIS.md` B1. **→ next is B2.**

**B2 — Targeted-floor direct test.** ✅ **DONE (CONFIRMED, Rust 10-seed).** Built the adaptive adversary +
diversity-maintenance floor; `targeted` uniquely holds the stable-adaptive regime (adapt LOW, survives
8/10) where `uncond` maxes adapt 1.50 (10/10, life-support/stasis) and `none` is a gamble (extinct 3/10).
Reproduces Python Exp 9 in bit-exact Rust → **word `targeted-floor` confirmed**; feeds UCF. First
word-schema entry. See CONSTELLA_TO_EXPERIMENTS.md. **→ next is B4 (reuses this floor code).**

**B3 — `entrainment` / rhythm-lock.** 🔧 **IN PROGRESS (mechanic built, needs redesign — no clean result
yet).** Built the `b3` mode: endogenous clock (period = `genome[3]`), periodic strike, `CALIB` flag for
the nature(endogenous) vs nurture(calibrated) arms. **Honest diagnosis — a real design tension, not a
tuning miss:** a strike weak enough to survive is *tankable* (energy recovers ~5·P between strikes → prep
never selects); a strike lethal enough to select *kills newborns* (born at random phase → die at first
strike before they can entrain) → collapse. No damage level cleanly selects entrainment with a
damage-avoidance design. It *did* show the framing's core: **pure NATURE (endogenous clock) can't hold
entrainment** — phase drifts across generations (nurture/calibration is needed to re-align). **Redesign
for next session:** (a) make the periodic event a **RESOURCE pulse to harvest** — prepared agents gain,
missing it lowers fitness *gradually* (no lethality, no newborn massacre → clean gradient selection);
and/or (b) **inherit phase** (`child.clock = parent.clock`) so entrained lineages stay aligned. · word:
`entrainment` (candidate, unearned) · feeds: **Civic Tome**.
  ✅ **DONE (CONFIRMED, 2026-07-09, Rust 8-seed).** The resource-harvest redesign worked: internal period
  locks to the environment and tracks it (100% at P=40/60, 15/16). Nature/nurture gradient robust
  (neither 0.64 < nature 0.84 < nurture 0.97 < both 1.02 harvest/agent/pulse). Boundary: fails when the
  rhythm is optional (P≥80). **Word `entrainment` confirmed** — third schema entry. **→ BATCH B1–B4
  COMPLETE.**

**B4 — `founding-diversity`.** ✅ **DONE (CONFIRMED, Rust 10-seed, reproduces Exp 8b).** diverse+targeted
survives 8/10 (adapt low) vs mono+targeted 5/10 (adapt high — predator specializes despite the floor).
Founding diversity is load-bearing; the floor maintains a reserve but can't reliably create one late
(boundary: occasionally bootstraps). **Word `founding-diversity` confirmed**; feeds "Diversity Before
Crisis". See CONSTELLA_TO_EXPERIMENTS.md. **→ next is B3 (entrainment, standalone build).**

*Banked out of this batch (return later): interface→diversity (needs neutral-tag redesign),
cultural-transmission re-test, Penumbra dissolution mechanic.*

---

## 🎯 NEXT BATCH C1–C4 (toward 20) — set up 2026-07-09
*Strategy: **reuse the B2 adaptive adversary** (built) to cheaply mine two un-covered Constella
mechanisms — Astris and Penumbra — then take on two fresh builds. Each: hypothesis · candidate word ·
Constella mechanic · receipt · cost. Order = cheapest/most-likely-to-confirm first.*

**C1 — `adaptation-speed-limit` (Astris/Auctor · reproduces Exp 5).** ✅ **DONE (CONFIRMED, Rust 12-seed
ratio grid).** Survival falls with adversary `RISE` (10/14→6/14 above ≈0.015) AND rises with agent `MUT`;
the ~50%-survival threshold `RISE` **scales with `MUT`** (0.5→0.01, 1.0→0.02, 2.0→0.05) → collapse is the
adversary/agent speed **ratio**, not the absolute rate. Added a tunable `MUT` (mut_scale; base hash
preserved). **Word `adaptation-speed-limit` confirmed** → feeds `tokens_astris_auctor.md`. See
CONSTELLA_TO_EXPERIMENTS.md. **→ next is C2 (strategic-dissolution).**

**C2 — `strategic-dissolution` (Penumbra Accord · reproduces Exp 7).** Reuse b2's adversary; add a
DISSOLUTION arm — when `adapt` is high the population can abandon the contested defense (go "naked") so the
adversary has nothing to specialize against → `adapt` decays → restore later. · hypothesis: dissolve-and-
restore drives `adapt`→0 and **out-survives persistence in a losing defense** (Exp 7's de-adapt-to-0). ·
word: `strategic-dissolution` · feeds: **Penumbra Accord** (formal dissolution pathway) · receipt:
dissolution arm drives adapt→0 + survives where the persist arm collapses, multi-seed. **Reuses b2 (small
add).**

**C3 — `cultural-transmission` (honest re-test of Exp 8 · failed 2/4 in Python).** Fresh mechanic: agents
horizontally copy a successful neighbor's strategy (not only genetic inheritance). · hypothesis:
transmission accelerates adaptation / raises survival vs pure inheritance. · word: `cultural-transmission`
(confirm **or** honest refute) · feeds: a knowledge-sharing mechanic (Civic Tome memory family) · receipt:
faster adapt / better survival reproducibly — or a clean null. **Fresh build (moderate).**

**C4 — `interface → diversity` (the amphiphile kernel).** The spatial-niche / neutral-tag redesign to test
the **untested diversity-half of the predictability law**: does a persistent spatial interface *maintain*
the diversity a uniform cycle destroys? · word: `interface-diversity` (completes diversify-vs-converge) ·
feeds: the predictability law + core_framework · receipt: interface arm holds diversity where the uniform
flip converges. **Hardest build — the amphiphile seed's testable core.**

*Deferred: diversity-reserve v7 (largely covered by B4/`founding-diversity`); Astris funded-floor variants;
Penumbra gamer-lifecycle (transitional-scaffolding).*

---

## ✅ RUN IN ALIFE (the sandbox — deterministic, bit-exact, reproducible)

1. ~~**exp3 — anticipation emergence.**~~ ✅ **DONE** — earned **`anticipation`** as confirmed word #2
   (conditional on global pressure; V1 local refuted the "89.2%", V2 global confirmed 5/5, V3 mapped
   the open reaction repertoire). Full log in `CONSTELLA_TO_EXPERIMENTS.md`. Seasons (Steps 1–4) also
   complete — see the RESUME POINT above.

2. **Evolution-as-entrainment (from the consolidated seeds — chronobiology-anchored).** Does a
   *periodic* environment select for traits that phase-lock with its rhythm (lower energy cost,
   higher fitness)? exp3's wave-anticipation is a special case; generalize it (light/dark, seasonal,
   tidal rhythms). Candidate word: **`entrainment`** / `rhythm-lock`. Natural next after exp3.

3. **The Constella civic cluster (the original goal — some already confirmed).**
   - ✅ **Civic floor** (confirmed word #1): floor→resilience, dignity-vs-diversity split,
     targeted-floor-fires-reserve, accountability-must-throttle-growth-not-survival.
   - **Diversity-reserve v7** — open frontier: does *founding diversity* (stop seeding a
     monoculture) let the targeted floor reach the stable-adaptive regime?
   - **Penumbra** — does a conflict-resolution mechanic beat pure suppression? (design-only → test)
   - **Cultural transmission** (exp8, failed 2/4 in Python) — re-test in Rust, honestly.
   - **Astris/Auctor** — contribution↔accountability (funded floor / reproduction-throttle variants).

4. **Scale-invariant transfer (the method itself)** — testable *within* sandbox scales for specific
   mechanisms; candidate *methodological* word. Lower priority; do after 1–3 give it targets.

## 🏦 BANKED — needs a proper lab / hardware / field (do NOT sandbox these)
Recorded in `research-notes/`; here so we don't confuse them with sandbox work.

- **Cross-scale physics stubs** (already designed): atom-as-planet (spectroscopy, NIST/CERN),
  body-cosmos (MRI/anthropometrics — *numerology risk, pre-register*), breath-tidal (HRV, coastal).
- **Acoustic levitator** — buildable (~$50 hardware), but electronics, not ALife.
- **Coalescence / analog-horizons invariant-transfer** — fluid/optics sim or bench, not ALife.
- **Tectonic standing-waves** (InSAR geophysics), **bio-sonar / atmospheric echo** (human/field
  trials), **mitochondrial/Warburg** + **cymatic microbiome** (wet lab).
- **Consciousness-as-waveform** — contemplative/philosophy, not an experiment (the ALife emergence
  work is its materialist counterpart, not a test of it).

## Immediate focus
**Finish exp3 Stage 5.** The whole port was building to it; every piece is validated bit-exact.
It's the shortest path to your second confirmed word.
