# Constella concepts → experiments → FAITHH vocabulary
2026-07. Answers both: "run Constella concepts into experiments" AND "a basic language
set for FAITHH." They're one pipeline: each Constella concept becomes a testable
experiment; each *confirmed* result becomes a word FAITHH is allowed to use.

> **This file holds the RECEIPTS. The CLAIMS live in the constitution repo:**
> `constella-framework/docs/governance/alife_evidence_mapping.md` (the bridge doc that maps
> these findings to constitutional mechanisms). The two are the two halves of the testable
> whitepaper — keep them in sync. See also `constella-framework/docs/governance/INDEX.md`.

## The pipe (one direction, three stages)
```
Constella concept  →  ALife experiment (hypothesis + mechanic)  →  confirmed result  →  FAITHH vocabulary term
   (top-down claim)        (bottom-up test, seeded, at scale)         (receipt)            (a word with a receipt)
```
A concept only earns a place in FAITHH's language when the sandbox *reproduces* it. Until
then it's `speculative` — a claim, not a word.

## Concept → experiment map
| Constella concept | The testable question | Mechanic | Success = | Exists? |
|---|---|---|---|---|
| **Universal Civic Floor** | does a stable minimum-provision floor emerge, and does it raise whole-population survival? | energy floor / redistribution rule | pop survival ↑ vs no-floor control, reproducibly | exp6_ucf_floor (Python) |
| **Diversity as survival trait** | do populations that preserve genome diversity outlast monocultures under shifting pressure? | diversity floor + changing selection | diverse pops survive shifts monocultures don't | exp9_diversity_floor (Python) |
| **Cultural transmission (the Tome)** | can learned strategy pass between agents & persist across generations? | protocol memory + transmission range | transmission + generational persistence both hold | exp8_cultural (Python) — **failed 2/4 last run** |
| **Penumbra (dissent→accountability)** | does a conflict-resolution mechanic outperform pure suppression? | signal/redirect vs suppress | resolution pops beat suppression pops | design only |
| **Astris/Auctor (contribution↔accountability)** | do reward+accountability pairs stay stable, or does one collapse? | dual-token contribution accounting | neither starves nor runs away | design only |
| **Proof of Life (liveness)** | (already prototyped separately — attestation heartbeat) | — | — | homelab/proof-of-life |

## The starter FAITHH vocabulary (tiered — this IS the language set)
Words the sandbox can already produce, with their current tier:
- **CONFIRMED (base ecology — earned, usable now):** *carrying-capacity*, *predation*,
  *thermal-death*, *selection*. (From the validated exp0 + the exp5 extraction:
  predation & thermal death recur across seeds.)
- **CONFIRMED (civic floor — earned 2026-07, Rust multi-seed; see consolidated section below):**
  *civic-floor* (→ resilience); *dignity-floor* vs *diversity-maintenance-pulse* (two distinct
  mechanisms); *diversity-reserve* (a targeted floor lets a preserved minority become the winner
  when conditions shift — fires reproducibly, robustness bounded). Plus negatives:
  *unconditional-floor → stasis* and *accountability-as-survival-drain → collapse*.
  **BOUNDARY (seasons arm, 2026-07, Rust 16-seed × 2 regimes):** the floor's benefit is
  *context-dependent*. Under **cyclical/seasonal** drain (not acute directional shock), moderate
  floors give **no** survival benefit (within noise of baseline in both sharp & slow seasons) and in
  sharp winters *lower* mean population via food **overshoot** (feed a weak crowd → depleted larder →
  harder crash). Only a *maximal* floor (= reproduction threshold, i.e. abolishing winter mortality)
  reliably helps. → the floor protects against adversity that **removes variation**, but is
  marginal-to-harmful against adversity that **starves a crowd.** `seasonal-floor` refuted as a clean
  positive; the artifact is the boundary. (SEASONS_EXPERIMENT_SPEC.md, Step 3.)
  **B2 CONFIRMATION (adaptive adversary, 2026-07-09, Rust bit-exact, 10 seeds) — reproduces Python Exp 9.**
  Ported the adaptive predator (specializes against the dominant defense `genome[6]&7`, `adapt` 0→1.5)
  + a diversity-maintenance floor. First entry in the WORD SCHEMA:
  - **word:** `targeted-floor` (→ stable-adaptive regime)
  - **claim:** a floor that preserves only MINORITY strategies keeps an adaptive adversary from ever
    specializing (adapt stays low) → the system stays both ALIVE and ADAPTIVE.
  - **tier:** confirmed (Rust, 10-seed).
  - **holds when:** an adaptive/co-evolving adversary + founding diversity. Targeted arm: adapt LOW
    (6/10 seeds ≤0.13; median ~0.12) and survives 8/10.
  - **fails when:** a non-adaptive stressor (nothing to keep from specializing); and it is not
    bulletproof — 2/10 seeds still extinct.
  - **receipt:** `alife-core b2 <seed> 20000` with `FLOOR=targeted`; adapt table in the B2 log.
  Paired negatives, both RECONFIRMED in Rust here:
  - `unconditional-floor → stasis`: adapt maxes to **1.50 in 10/10 seeds** — survives (10/10) but only
    as life-support; the adversary fully specializes → never adaptive. Buys survival at the cost of
    adaptiveness.
  - `no-floor → gamble`: unreliable — adapt ranges 0.10–1.50, **extinct 3/10.**
  The UCF tension, quantified: **survival** uncond(10) > targeted(8) > none(7); **adaptiveness**
  targeted ≫ none > uncond. Only the *targeted* floor reaches the stable-ADAPTIVE regime (alive AND
  adaptive). → feeds `ucf.md` (evidence-bridge update pending).
- **CONFIRMED (anticipation — earned 2026-07, Rust, conditional on global pressure):**
  *anticipation* — functional foresight (shield BEFORE a wave is sensable) arises UNSEEDED and fires
  reproducibly (**5/5 seeds under GLOBAL waves**; V2). Caveats: episodic/boom-bust, extinction risk
  under stealth, high variance. NOT reproducible under the original exp3's LOCAL waves (V1: 2/5) —
  which is why the unbacked "89.2%" never held. Word #2, with boundary conditions.
- **SPECULATIVE (Constella-target — claims, not yet words):** *diversity-preservation* (Python
  exp9 only), *cultural-transmission* (failed 2/4), *parasitism* (only 4/5 seeds), *arms-race*.
  These become vocabulary ONLY when a seeded run at scale confirms them.
- **The rule FAITHH obeys:** it may reason WITH a confirmed term as a lens; a speculative
  term must be spoken as "this looks like X (unconfirmed)" — never as fact. And any
  mapping to *humans* stays speculative regardless (a sim word is not a human truth).

## The honest gate (what's needed to actually run these)
- The **base ecology (exp0) is ported to Rust** and runs bit-exact, ~70× faster.
- The **Constella-target experiments (exp6/8/9) use special mechanics** (waves, floors,
  transmission) that are **NOT yet ported to Rust** — they run in Python (slow) today.
- So: to confirm the Constella concepts at scale, either (a) run them in Python (works,
  ~33min per 50k run), or (b) port their mechanics to Rust (the base tick is the hard
  part and it's done — adding a floor/diversity/transmission rule is incremental).

## Recommended order
1. Pick ONE concept with an existing experiment (UCF floor = exp6, cleanest).
2. Run it seeded across N seeds (Python is fine for one concept).
3. Run pipeline_extract.py → is "civic-floor" CONFIRMED or SPECULATIVE?
4. If confirmed → it enters FAITHH's vocabulary as a word with a receipt, and it's the
   first bottom-up *evidence* for a Constella claim. If not → honest counter-evidence.
5. Repeat per concept. The vocabulary grows only by confirmation.

## Jonathan's core speculation — sharpened (the floor + its users → system health)
Captured 2026-07. Distinct from exp6 (which tests survival: "does the floor prevent
collapse?"). This is a deeper *health* hypothesis:

> A civic floor improves whole-system health not through charity, but because the
> floor-supported members are a **diversity reserve** that buffers the system against
> shocks. The "weak" hold variants that become valuable when conditions shift. Remove
> the floor → lose the reserve → the system becomes **efficient but brittle.**

**Why it's the sharper form:**
- Reframes the floor-users as load-bearing (the reserve), not a cost. The weak are the
  system's *memory of alternatives*.
- Unifies exp6 (floor) + exp9 (diversity): the floor promotes health BY preserving the
  diversity reserve. One mechanism, two sides.

**Testable design (a real experiment, not yet built):**
- Two populations under periodic environmental shocks: FLOOR (weak supported) vs
  NO-FLOOR (weak die).
- Measure "system health": post-shock recovery speed, genome diversity retained,
  population stability (low boom-bust variance).
- Prediction: FLOOR population is MORE resilient (better health).
- The load-bearing test: does the post-shock rebound come from descendants of
  *previously floor-supported* (low-energy) agents? If yes → the weak were provably
  load-bearing → the speculation confirmed with a receipt.
- Null: floor → free-riding/bloat/no recovery advantage (exp6's null).

**Tier:** speculative until run seeded at scale. If confirmed, it's the strongest possible
bottom-up evidence for the UCF — and a genuine, non-obvious finding worth publishing.

## ★ THE CIVIC FLOOR EXPERIMENT — consolidated result (2026-07)
The first Constella concept taken to CONFIRMED. The detailed run log (FIRST–SEVENTH RESULTS below)
is the receipts; this is the single finding they add up to.

**Question tested:** does a civic floor improve whole-system health under shocks, and is a
*diversity reserve* the mechanism? (Jonathan's core speculation, above.)

**Confirmed (Rust, bit-exact, multi-seed):**
1. **Floor → resilience.** With a floor the system survives shocks reliably (5/5 seeds); without
   it, survival is a gamble (extinct 3/5). The floor is insurance. (v1)
2. **Two floors, not one.** A *dignity/survival floor* (unconditional by right, but funded &
   bounded) is a DIFFERENT mechanism from a *diversity-maintenance pulse* (targeted, threshold-
   activated). Conflating them causes every unconditional-floor failure → split into `ucf.md`. (v3)
3. **Unconditional floor → stasis/collapse.** Rescuing everyone freezes the composition (Rust:
   787/43 forever) or, with an adaptive predator, collapses it (Python exp6, tick 7,410). (v3)
4. **The targeted floor FIRES the reserve.** Under a directional shock, a floor that protects only
   the under-represented lets a preserved minority *become the winner* when its regime returns
   (reproducible across 3/5 seeds, up to 5 turnovers in one). Adaptation via the reserve is REAL —
   the thing the unconditional floor never does (0/9). (v4)
5. **Accountability must throttle growth, not survival.** A cap implemented as a survival drain
   collapses the system (5/5); it must limit reproduction/advantage, never existence — the Astris
   vs dignity-floor separation, confirmed empirically. (v5–v6)

**Frontier (NOT achieved — this is v8, a separate model):** a *robust* stable-AND-adaptive floor.
The targeted floor's turnover is fragile (survives 3/5) because of a **structural turnover
bottleneck** — at each regime flip the dominant group crashes while the thin reserve rebuilds from
a handful (pop 2–5). Softer shocks trade adaptation for survival; larger founding populations wash
out; a share-based accountability cap makes it worse (v5–v7). Cracking it needs a richer model
(continuous reserve replenishment, higher carrying capacity, or more strategies) — a NEW experiment.

**Vocabulary earned:** *civic-floor*, *dignity-floor* / *diversity-maintenance-pulse*,
*diversity-reserve* (fires; robustness bounded), and the negatives *unconditional-floor→stasis* and
*accountability-as-survival-drain→collapse*. First bottom-up, reproducible evidence for a Constella
mechanism.

---
### Detailed run log (the receipts for the consolidated finding above)

## FIRST RESULT — UCF floor experiment (Rust, 2026-07)
Built in alife-core (`run ... ucf <seed> <ticks>`): FLOOR vs NO-FLOOR arms, same seed,
periodic shocks (energy sources relocate + famine every 2000 ticks). 5 seeds, 10k ticks.

| seed | FLOOR final (div) | NO-FLOOR |
|---|---|---|
| 42 | 788 (67) | **EXTINCT** |
| 7 | 832 (69) | 916 (survived, beat floor) |
| 100 | 830 (72) | 836 (survived) |
| 123 | 832 (71) | **EXTINCT** |
| 2024 | 825 (72) | **EXTINCT** |

**The finding (reproducible):** the floor **trades peak efficiency for resilience.**
- WITH floor: survives 5/5, consistent ~800, never crashes. Resilient.
- WITHOUT floor: extinct 3/5; but when it survives it can *beat* the floor. Efficient but BRITTLE.

This **confirms the "efficient but brittle" half of Jonathan's hypothesis with a receipt:**
a civic floor makes whole-system survival reliable where, without it, survival is a gamble.
The floor is insurance — it never wins biggest, but it never dies.

**Honest caveats (what v2 needs to make it airtight):**
1. The floor here is *free* (rescue-to-30, ~immortality-lite; 200k+ rescues/run). A realistic
   floor has a commons-pool cost. The resilience result likely survives a cost, but test it.
2. The DIVERSITY-RESERVE *mechanism* is not yet proven — we show floor→resilience (outcome),
   not that resilience comes FROM the preserved reserve. v2: the lineage test (does post-shock
   rebound descend from previously-floored agents?).
3. Single environment/shock type. Vary shock magnitude & interval.

**Tier: the OUTCOME (floor→resilience) is CONFIRMED (5 seeds, reproducible). The MECHANISM
(diversity reserve) stays speculative until the lineage test.** Vocabulary earned:
*civic-floor → resilience* (confirmed); *diversity-reserve* (still speculative).

## SECOND RESULT — the MECHANISM test (honest NEGATIVE + diagnosis)
Built `alife-core mech <seed>`: floor arm, snapshot genome-freq before each shock, then
1000 ticks post-shock ask "was the new dominant a PRE-shock minority?" (= reserve load-bearing).

seed 42, 20k ticks, 9 shocks: **0/9 reserve-driven.** The SAME dominant genome (~65% of pop,
515/788) wins before AND after every shock. The winner never shifts; preserved minorities
never rise.

**Finding: the diversity-reserve MECHANISM is NOT operating as hypothesized.**
- Outcome (floor→resilience) still holds — but it runs on HEADCOUNT (buffering), not
  variant-supply. This is exp6's null partially winning on the *mechanism*.
- So "the weak are load-bearing because they hold variants that become winners" is NOT
  demonstrated. Honest counter-evidence, with a receipt.

**Diagnosis (why the reserve wasn't tested, not just false):** the v1 shock is a UNIFORM
stress (relocate sources + famine, everyone hit equally) → a GENERALIST wins every time →
no *directional* selection → no previously-suboptimal variant ever becomes optimal → the
reserve has no moment to matter. The diversity-reserve hypothesis specifically predicts
benefit under DIRECTIONAL shifts. v1 never creates one, so it can't confirm OR refute the
mechanism — it only shows the floor buffers uniform stress by headcount.

**What v3 needs (the real mechanism test):** a DIRECTIONAL shock that alternates which TRAIT
is optimal (e.g. HOT regime → shield-users win; SCARCE regime → mover/conserver-users win),
so different genomes win in different regimes. Then: does the floor-preserved off-regime
variant rise when its regime returns? That requires enabling hazard/threat regimes (not in
base exp0) — a genuine next experiment, not an edit. Also: the seeded near-monoculture
(genome[5]=4,genome[6]=2 fixed for all founders) thins the act-slot reserve from the start.

**Tier update:** *civic-floor → resilience* CONFIRMED (headcount-buffering, receipt).
*diversity-reserve as the mechanism* → **NOT SUPPORTED under uniform shock**; UNTESTED under
directional shock (v3). Do not claim the reserve mechanism until a directional shock confirms it.

## THIRD RESULT — DIRECTIONAL shock: the floor has a CALIBRATION regime
Built `alife-core dmech <seed> [nofloor]`: shock now FLIPS which trait-group (regulate op
genome[7]&7: <4 vs >=4) the environment favors, keyed to standing variation. This is the
first shock that creates DIRECTIONAL selection (a different trait becomes optimal), so the
reserve *could* be invoked. seed 42, 20k ticks, 9 flips.

**FLOOR arm — STASIS.** Composition frozen at grp0=787 / grp1=43 across ALL 9 flips, div=74
frozen. The unconditional floor (rescue-to-30) keeps the disfavored alive — but ALSO shields
the 787-strong incumbent from the selection meant to demote it. The reserve is *preserved but
inert*: it never dies and never rises. The system survives forever, adapts never.

**NO-FLOOR arm — EXTINCT at tick 2035** (35 ticks after the first flip). Turnover was allowed,
but the disfavored group died during regime 0, so when the regime flipped there was no reserve
to take over → collapse. Brittle, exactly as predicted — just fatal rather than adaptive.

**The real finding (a design law, not a yes/no):** the UCF has a **calibration regime.**
- Floor too strong (unconditional immortality) → **stasis**: survives, but the incumbent is
  over-protected and the reserve can't rise. No adaptation.
- Floor absent → **brittleness**: reserve is lost, first directional shock is fatal.
- The diversity-reserve mechanism (reserve RISES when its regime returns) requires a *middle*
  floor — one that preserves the weak WITHOUT freezing selective turnover. That floor was not
  built here; both extremes miss it.

**Also a precondition, revealed:** the founders are seeded as a near-monoculture (genome[5]=4,
genome[6]=2 fixed; one genome ≈62%), so the reserve group started tiny (43) — too small to take
over in one regime window even if selection favored it. **Standing diversity is a prerequisite
for the reserve to matter at all.**

**This refines "headcount buffering is a good-enough ethos":** headcount buffering is real and
sufficient for SURVIVAL (floor beats no-floor every time). But taken to the unconditional
extreme it ossifies the system. A civic OS needs floor-for-survival AND preserved turnover —
the floor must keep the weak alive without making the strong un-removable.

**Tier update:** *civic-floor → resilience* CONFIRMED. *civic-floor → stasis when unconditional*
NEW, CONFIRMED (frozen composition, receipt). *diversity-reserve enables adaptation* → still
NOT demonstrated; now known to need (a) a calibrated (non-freezing) floor, (b) standing
diversity, (c) directional selection — v4. Do not claim the reserve mechanism yet.

## FRAMEWORK NOTE (design, 2026-07) — the floor revealed an AXIS, not a knob
The three floor states (none / middle / unconditional) with opposite failures
(brittle / — / frozen) are the first instance of a general pattern worth naming:

**Resonant-boundary pattern:** a civic mechanism is a control parameter with two failure
extremes and a healthy middle; the interesting behavior lives at the boundary, not the
extremes. Expect other Constella concepts to share this shape.

**"Must every concept roll into the three floor states?" → No.** That's a combinatorial
explosion; most cells are uninformative. Rule: cross a concept with the floor states ONLY
where it plausibly interacts with survival/selection. On-axis concepts (diversity,
accountability, contribution) are tested WITH the floor; orthogonal ones (some cultural-
transmission) standalone.

**The floor's axis = survival ↔ removability.**
- Floor = LOWER bound (nobody dies). No floor → brittle (death from below).
- The stasis failure = missing UPPER bound (incumbents un-removable). Total floor → frozen.
- The upper bound already has a Constella name: ACCOUNTABILITY (Penumbra / Astris-Auctor) —
  the mechanism that demotes/removes entrenched incumbents.

**Reframed v4 (replaces "calibrate the floor's strength"):** the "middle floor" is likely
NOT a weaker floor — it's a FULL floor PLUS accountability. You don't buy turnover by letting
the weak die; you buy it by keeping the strong removable. Floor + Accountability = the middle
regime where survival AND adaptation coexist and the diversity reserve can finally rise. This
fuses UCF + Penumbra into ONE two-boundary system and is the real v4.

**Proposed next experiment:** add an accountability mechanic (incumbent demotion — e.g. the
dominant lineage pays a rising entrenchment cost, or top-share agents face elevated drain) ON
TOP of the full floor + directional shock. Prediction: composition un-freezes (turnover
returns), survival holds (floor on), the favored reserve rises after each flip → the reserve
mechanism finally fires. Tier: design/speculative until run seeded.

## FOURTH RESULT — the targeted pulse floor (v4): the mechanism fires, then overshoots
Built `alife-core dmech <seed> <ticks> pulse`: a threshold-activated floor (Exp 9 style) that
rescues ONLY the under-represented group (share < 30%), leaving the dominant incumbent exposed to
selection. Directional penalty softened 8→3 so the base population self-sustains (at 8 the pop was
100% floor-dependent — an Exp-6 "inflated past carrying capacity" artifact; note v3 above used 8).

Three arms, seed 42, 20k ticks:
- **NO-FLOOR:** collapses to pop=1 (lone monoculture survivor). Brittle.
- **UNCONDITIONAL FLOOR:** 830 agents, composition FROZEN (774/56 across all 9 flips). Survives,
  never adapts. Stasis (confirms v3).
- **TARGETED PULSE:** at the first flip **the reserve ROSE** — winner shifted to the newly-favored
  group (grp1 5→916), **reserve-driven: TRUE**. The mechanism the unconditional floor never fired
  (0/9) fired here. BUT the turnover overshot into the opposite monoculture (99%), leaving no
  reserve for the next flip → EXTINCT at tick 4102.

**Finding:** the targeted floor CAN do what the unconditional floor cannot — let a preserved
minority become the winner when its regime comes (first positive evidence for the diversity-reserve
mechanism). But a bare threshold pulse OVERSHOOTS: protecting the weak (floor) without limiting the
strong's runaway growth (accountability) produces violent monoculture swings that collapse across
repeated shocks. This empirically reproduces the evidence-mapping "gate must be calibrated"
principle (Exp 7's too-tight gate) AND confirms this session's floor+accountability thesis: the
floor alone is a *half*-mechanism. The stable middle regime needs BOTH — targeted floor to preserve
the reserve AND accountability to cap incumbent runaway.

**Tier:** *targeted-floor enables turnover* — SUPPORTED (one clean reserve-driven flip vs 0/9 for
unconditional). *stable adaptive floor* — NOT YET (overshoot→collapse; needs floor + accountability).
Honest: the mechanism is real but only half-built. **v5: add the majority cap (the Astris/Auctor
accountability analog) and re-test.**

## FOURTH RESULT — HARDENED (multi-seed, 2026-07-07)
Ran the three arms across 5 seeds (42, 7, 100, 123, 2024), 20k ticks each. The multi-seed sweep
REVISES the single-seed (42) v4 conclusion:

| seed | NO-FLOOR | FLOOR (uncond) | PULSE (targeted) |
|---|---|---|---|
| 42 | lone survivor | frozen 774/56 | reserve fired 1x → extinct @4102 |
| 7 | lone survivor | frozen 787/43 | reserve fired 2x → survived |
| 100 | lone survivor | frozen 578/40 | no fire → survived |
| 123 | extinct @2068 | frozen 804/27 | reserve fired 5x → survived |
| 2024 | extinct @4073 | frozen 800/30 | no fire → extinct @2061 |

**Revised finding (multi-seed changes the story):**
- FLOOR (unconditional): frozen composition in ALL 5 seeds — stasis is robust/reproducible.
- NO-FLOOR: fragile — 2/5 hard-extinct; the rest reduced to a lone monoculture survivor.
- PULSE (targeted): the reserve mechanism FIRES in 3/5 seeds (1x, 2x, 5x) — so "the targeted
  floor lets a preserved minority become the winner" is REPRODUCIBLE across seeds, not a
  seed-42 fluke. Seed 123 shows the GOAL reached: 5 turnovers WITH survival (sustained
  adaptation, no fatal overshoot). But survival is only 3/5 and highly variable; the bare pulse
  still overshoots to extinction in 2/5.

**Honest correction:** the single-seed v4 write-up ("fires then overshoots → extinct")
over-generalized from seed 42, which happened to be a failure case. Multi-seed: the mechanism is
real and reproducible, and CAN produce sustained adaptive turnover (seed 123), but the bare
targeted pulse is UNSTABLE (high variance, fatal overshoot 2/5).

**Tier:** *targeted-floor fires the reserve (adaptive turnover)* — CONFIRMED reproducible (3/5
seeds). *targeted-floor is stable/robust* — NOT YET (survives 3/5, overshoot fatal 2/5). v5 =
add the accountability cap to damp overshoot and make turnover reliable. (Metric caveat:
NO-FLOOR "survived" = a lone survivor, effectively dead; a finer liveness metric would score
it worse.)

## FIFTH RESULT — accountability cap (v5): a survival-drain cap COLLAPSES the system
Built `dmech <seed> <ticks> capped`: the v4 targeted pulse PLUS an accountability cap — an
over-represented trait-group (share > 0.70) pays an entrenchment energy drain/tick ("keep the
strong removable"). Tested CAP_PENALTY = 6 and 2, 5 seeds each.

| seed | PULSE (v4) | PULSE+CAP 6/tick | PULSE+CAP 2/tick |
|---|---|---|---|
| 42 | rd1, ext@4102 | ext@173 | ext@460 |
| 7 | rd2, surv | ext@154 | ext@313 |
| 100 | rd2, surv | ext@136 | ext@350 |
| 123 | rd5, surv | ext@242 | rd1, ext@4070 |
| 2024 | rd0, ext@2061 | ext@145 | ext@304 |

**Finding (honest NEGATIVE): a cap implemented as a SURVIVAL DRAIN collapses the system at every
magnitude** — 5/5 extinct, mostly BEFORE the first shock (tick ~150–460), because the cap fires
during normal early consolidation (the growing population naturally drifts one group past 70%)
and drains the majority to death. The pulse floor can't save it: the floor protects the
under-represented, but the cap kills the over-represented — and "over-represented" is *most of the
population* during founding.

**Two constitution principles reproduced:**
1. *Accountability must not outpace adaptation* (exp5 complexity/adaptation ratio; the Astris
   decay-rate rule): a 6- or even 2-energy/tick cap outpaces adaptation → collapse.
2. *Tolerate early consolidation during the founding window* (evidence_mapping mechanism #3): the
   cap punishes the healthy founding majority, killing the system before it can stabilize.

**The deeper lesson (the Astris ↔ dignity-floor separation, empirically):** you cannot implement
accountability as a **survival threat** — it collides with the floor's job (guarantee existence).
Accountability must throttle **growth/influence** (reproduction, advantage), never existence. A
cap that *kills* is a category error. This is exactly the constitution's separation — Astris
(merit, decays/capped) vs dignity floor (survival, inviolable). The sandbox merged them
(cap = survival drain) and collapsed, confirming they must stay separate.

**Tier:** *accountability-as-survival-drain* — FALSIFIED (5/5 collapse). *stable adaptive floor* —
still open. v6: implement the cap as a REPRODUCTION throttle on the over-represented group (slow
runaway growth, never threaten survival) and/or engage only after the founding window.

## SIXTH RESULT — accountability cap redesigned (v6): still FALSIFIED; it's a BOTTLENECK, not runaway
After v5 (survival-drain) collapsed, v6 rebuilt the cap as a REPRODUCTION THROTTLE — an
over-represented group (share > 0.70) pays a surcharge on its reproduction threshold: slows
growth, never threatens survival (fixing v5's category error). Tested surcharge 60, 20, and
founding-gated 40 (engage only after tick 1800, per "tolerate founding consolidation"). Multi-seed:

| cap variant | survives (of 5) | vs bare pulse (3/5) |
|---|---|---|
| v6 throttle 60 | 0/5 | worse |
| v6.1 throttle 20 | 1/5 (lone survivor) | worse |
| v6.2 founding-gated 40 | 2/5 (tiny survivors) | worse |

**Finding: NO accountability-cap variant beats the bare targeted floor.** v6's throttle did fix
v5's founding-collapse (it survives the founding phase now), but every configuration — gentle,
harsh, founding-gated — still drops survival below the bare pulse and yields sicker, near-dead
survivors. The share-based accountability cap is FALSIFIED as the stabilizer in this sandbox.

**Why (the reinterpretation that matters):** the v4 pulse's overshoot-death is NOT "runaway a cap
can prevent." At a regime flip the former-dominant group crashes AND the newly-favored group
hasn't yet built up — a TURNOVER BOTTLENECK. Capping the winner makes the bottleneck *worse*
(fewer winners too). The cap targets the wrong failure mode.

**Constitutional read:** the sandbox is saying the THREE-mechanism system (founding diversity +
targeted floor + tolerance) matters more than bolting on a 4th accountability lever — consistent
with evidence_mapping ("all three together produced the only stable run, Exp 9"). Our seeded
near-monoculture (genome[5]/[6] fixed) gives THIN founding diversity, so every turnover is a
knife-edge.

**Tier:** *targeted-floor fires reserve* — CONFIRMED (v4). *accountability-cap stabilizes it* —
FALSIFIED across 4 variants (v5, v6, v6.1, v6.2). *stable adaptive regime* — still open; the
evidence now points to **founding diversity (mechanism #1)**, not accountability, as the next
lever. v7: increase founding diversity (stop seeding a monoculture) and/or soften the shock, then
re-test the bare pulse.

## SEVENTH RESULT — v7: shock-softening and founding-population both fail to give stable+adaptive
Made shock strength (DIR_PENALTY) and founding population (POP) runtime knobs (experiment-only;
base sim still bit-exact a2bb005395f79766). Bare pulse, verified against the v4 hardening baseline
(seed 42 ext@4102, seed 2024 ext@2061 — reproduced exactly, so the code is consistent).

**Lever 1 — softer shock (DIR_PENALTY 3→2→1):** the fatal seeds (42, 2024) now SURVIVE — but with
reserve-fires = 0. Softening the shock trades adaptation for survival: too harsh → turnover +
bottleneck death; too gentle → survival + stasis. Neither extreme is stable-AND-adaptive.

**Lever 2 — larger founding population (POP 50→150→300):** does NOT help. Under the harsh shock,
POP=150 still collapses at the SAME turnover bottleneck (necks to pop=2 at first recovery, extinct
@4068 vs POP=50's @4102). Initial count washes out; the bottleneck is structural.

**Process note (caught a false positive):** an earlier POP sweep reported "survived" for POP=50
under harsh shock, contradicting the verified baseline. Re-running without the `timeout` wrapper
showed the real result (extinct @4102) — the timeout had truncated output before the EXTINCT line.
The false positive was discarded; the verified numbers above stand.

**Diagnosis:** the turnover bottleneck is structural to this two-group directional design — at each
regime flip the whole previously-dominant group crashes while the thin reserve must rebuild from a
handful (pop 2–5), and with a marginal carrying capacity it usually can't. Bigger founding numbers
wash out; gentler shocks remove the turnover entirely.

**Tier:** *stable adaptive regime* — NOT ACHIEVED by shock-softening or founding-population. The
two-group model may be too brittle for a clean stable-adaptive regime. v8 candidates: continuous
reserve replenishment (higher genome[7] mutation, experiment-gated), higher carrying capacity (so
the thin reserve is still dozens not 2), or a richer multi-strategy environment. The verified core
finding stands: the TARGETED FLOOR fires the reserve (v4, 3/5) — adaptation is real but fragile;
robustness needs a thicker reserve at the moment of turnover, which neither v7 lever supplied.

## exp3 — ANTICIPATION EMERGENCE (Stage 5 result, 2026-07)
Full exp3 port complete (Stages 0–5; per-stage bit-exact vs Python, see VALIDATION.md). Two-arm
emergence test: Arm A reactive-only (STRICT emergence — anticipation can only arise by mutating
P1 PROC_THRESHOLD→PROC_PREDICT); Arm B seeded (10 anticipatory, diagnostic). Waves every 200 ticks.

**Arm B (diagnostic — instrument CONFIRMED):** seeded anticipatory agents fired 1906 negative gaps
(first t455). The instrument works — the anticipatory genome does produce shield-before-detection.

**Arm A (reactive-only, 5 seeds, 40k ticks):**
| seed | PROC_PREDICT evolved (peak) | negative-gap events (foresight fired) |
|---|---|---|
| 42 | peak 3 | 489 |
| 7 | peak 2 | 0 |
| 100 | never | 0 |
| 123 | peak 2 | 0 |
| 2024 | peak 1 | 68 |

**Honest verdict — `anticipation` NOT confirmed at the strict-emergence bar:**
- The anticipatory op (PROC_PREDICT) **EVOLVES unseeded in 4/5 seeds** — the genetic substrate for
  foresight reliably arises from mutation. Real positive.
- But **functional anticipation (negative gaps — shield fired BEFORE the wave was sensable) fires
  in only 2/5 seeds**, always as a tiny minority (peak 1–3 of ~900). Bar was ≥3/5 → not met.
- So: capacity-for-anticipation emerges reliably; functional anticipation is rare/seed-dependent;
  it never sweeps.

**Why (mechanistic):** faithful to exp3's design, waves sweep only the LEFT ~third (interval 200 <
crossing-time 600 → single-wave overwrite) and the genomes have no MOVE op → agents don't migrate.
Only a wave-zone minority feels selection for anticipation, and it's too weak/local to fire reliably
or spread. **This REFUTES/reframes the asserted "89.2% predictive shielding"** (which had no
receipt — the docs already flagged it): the real reproducible behavior is 4/5-evolve, 2/5-fire,
never-dominant.

**Tier:** *anticipation-capacity-emerges* — SUPPORTED (PROC_PREDICT evolves 4/5, unseeded).
*anticipation (functional, strict-emergence)* — NOT CONFIRMED (2/5 fire; bar ≥3). Honest negative
on the strong claim, real positive on the substrate. **v2 to test properly:** global waves (cross
the full world) + a MOVE op so agents migrate → real non-local selection pressure for anticipation.

**Validation note:** per-stage pieces are bit-exact vs Python; the Stage-5 harness is a faithful
port of the Python exp3 loop and deterministic-in-Rust (reproducible receipt), but the full run has
not itself been bit-exact-validated against a Python full run.

## exp3 V2 — GLOBAL WAVES: `anticipation` CONFIRMS (conditional on global pressure)
V1 finding: functional anticipation fired only 2/5 because selection was weak/LOCAL (waves swept
only the left third; genomes can't move). V2 tests that mechanistic hypothesis directly: make waves
GLOBAL — interval 1300 > max crossing (480/0.4=1200) so every wave crosses the WHOLE world and all
agents face periodic waves, incl. **stealth waves (30%, instant death, survivable ONLY by
prediction)**. Same reactive-only strict-emergence setup (no seeded anticipators).

**Arm A (reactive-only, global waves, 5 seeds, 40k ticks):**
| seed | finalPop | PROC_PREDICT now/peak | neg-gap events |
|---|---|---|---|
| 42 | 941 | 14 / 4 | 24 |
| 7 | EXTINCT@19804 | 0 / 51 | 5 |
| 100 | 891 | 0 / 2 | 1884 |
| 123 | 852 | 0 / 95 | 237 |
| 2024 | 1009 | 0 / 267 | 576 |

**Verdict — `anticipation` CONFIRMED at the strict-emergence bar (conditional):**
- Functional anticipation (negative gaps) fires in **5/5 seeds** (vs 2/5 in V1) → clears the
  pre-registered ≥3/5 bar. Foresight emerges unseeded, works, and reproduces under global pressure.
- PROC_PREDICT is STRONGLY selected — peaks 50–267 (vs V1's 0–3). Under pervasive threat (esp.
  stealth), anticipation is a *dominant* response, not a rarity.
- **V1's mechanistic hypothesis VINDICATED**, and predicted in advance (not post-hoc): anticipation
  was rare in V1 *only* because selection was weak/local. This is hypothesis→test→confirm, not
  goalpost-moving — the two conditions are both receipts.

**Honest caveats (the shape of the confirmation):**
1. **Episodic / boom-bust, not a fixed trait.** Predictors bloom after waves, then drift away in
   the long safe gaps (interval 1300; mutation drifts P1 away) — high peaks, ~0 final counts.
   Anticipation is a **selected-on-demand** response. (Realistic: episodic threat → episodic adaptation.)
2. **Extinction risk (1/5).** Seed 7 died @19804 — global stealth waves can wipe a population
   *before* it evolves enough predictors. The pressure that selects for foresight can also kill first.
3. Magnitude high-variance (5–1884 events).

**Tier: `anticipation` → CONFIRMED as an emergent selected response** — functional foresight arises
unseeded and fires reproducibly (5/5) **under global/pervasive selection pressure** — with caveats:
episodic/boom-bust, extinction risk under stealth, high variance. **Conditional on global pressure;
the original exp3's LOCAL design (V1) is why the unbacked "89.2%" was never reproducible.** Word #2,
earned honestly, with its boundary conditions attached.

## exp3 V3 — FIGHT-OR-FLIGHT: the open reaction repertoire
Built the reactions OPEN (per Jonathan): the threat-response act-op (A1) is the repertoire —
SHIELD(defend) / FLEE(flight) / IDLE(freeze) / TOXIN(fight). Made FLEE wave-aware (run right, ahead
of an L→R front; agent speed 1 > wave 0.8 → can outrun). Modes: `open` (diverse A1 seeding +
reaction-distribution readout), `allflee/allidle/alltoxin` (isolate one reaction). Extensible — a
new reaction is just another op.

**Single-reaction survival under GLOBAL waves (3 seeds):**
| reaction | result |
|---|---|
| SHIELD (defend) | **robust** — 905/887/799 (3/3 survive healthy) |
| FLEE (flight) | **viable but riskier** — 900/874 survive, 1 near-collapse (s42=1) |
| IDLE (freeze) | marginal — survives some seeds, extinct others |
| TOXIN (fight) | inert (TOXIN_ACTIVE=false; no effect on a wave) |

→ **Both defense AND flight are viable strategies** (flight works by outrunning the wave); freeze is
marginal; fight is useless against this threat. Foresight (predict) is a refinement of shield.

**Diverse mix (`open`, 25% each) → MASS EXTINCTION (4/5 seeds; lone survivor = 3 shield agents).**
NOT because flee/shield are bad (they survive alone) — because the ~50% weak-reaction founders
(freeze/fight) can't survive the pervasive lethal waves, and their die-off drags the whole
population below viability before selection can favor the good reactions.

**The non-obvious finding (and a counterpoint to the diversity-reserve result):**
- *Trait* diversity HELPS under **shifting** conditions (the civic-floor reserve — a rare variant
  becomes the winner when the environment changes).
- *Reaction* diversity HURTS under a **constant lethal** threat with a clear best answer — the
  wrong-reaction majority dies fast and crashes the system before convergence.
- So: **diversity is a reserve when the future is uncertain; it's a liability when the right answer
  is known and being wrong is fatal.** When to converge vs. when to diversify is itself the design
  question. (Directly relevant to the civic framework's floor/diversity balance.)

**Tier:** *multiple viable defenses (shield ~ flee) under global waves* — SUPPORTED (single-reaction
runs). *reaction-diversity is fatal under a constant lethal threat* — SUPPORTED (open-mix 4/5
extinct). v3-next: mix ONLY viable reactions (shield+flee) to test flight-vs-foresight head-to-head
without the freeze/fight drag; add a real MOVE-toward-safety so flight is less of a corner-trap.
