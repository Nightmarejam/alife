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

**To continue, pick one:** (0) ★ metronome-vs-reprieve confirmation [teed up below]; (a) interface→
diversity kernel [closes Step 4]; (b) diversity-reserve v7; (c) entrainment (item 2 below);
(d) cultural-transmission re-test.

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

**B3 — `entrainment` / rhythm-lock.** Does a *periodic* environment select traits that phase-lock to its
rhythm (lower cost / higher fitness than non-locked)? Generalizes `anticipation` beyond waves (light/dark,
tidal, seasonal). · word: `entrainment` · feeds: **Civic Tome** (anticipation family) · receipt:
phase-locked genomes out-compete non-locked under periodic pressure, reproducibly. (Chronobiology-anchored;
was queue item 2.)

**B4 — `diversity-reserve` v7 / founding-diversity.** Does *founding* diversity (stop seeding a
monoculture) + a targeted floor reach the stable-adaptive "three-mechanism" regime? · word:
`founding-diversity` (load-bearing) · feeds: **"Diversity Before Crisis"** principle + UCF · receipt:
founding-diverse + targeted-floor stays adaptive where monoculture-founded doesn't. **Reuses B2's
threshold-floor.**

*Banked out of this batch (return later): interface→diversity (needs neutral-tag redesign),
cultural-transmission re-test, Penumbra dissolution mechanic.*

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
