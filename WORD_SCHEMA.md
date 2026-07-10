# Word schema — the canonical shape of an earned word

*The vocabulary that flows ALife sandbox → Constella (constitution) → FAITHH. Every earned word takes
this one shape, so the working ledger (`CONSTELLA_TO_EXPERIMENTS.md`), the constitution's evidence
bridge (`constella-framework/docs/governance/alife_evidence_mapping.md`), and the eventual training
dataset all share a single structure. "Emergent policy grounded in science, envisioned with the heart,
backed by receipts."*

## The schema (one entry per word)

| field | meaning |
|---|---|
| **word** | the term (kebab-case) |
| **claim** | one sentence: what it asserts |
| **tier** | see below |
| **holds-when** | the conditions/regime where it's true (with the numbers) |
| **fails-when** | the boundary — where it breaks, is marginal, or reverses |
| **receipt** | the reproducible command + seed count (`alife-core … <seed> <ticks>`) |
| **feeds** | the Constella mechanism/doc it informs |

A word without `holds-when` + `fails-when` is not done — the boundary *is* part of the definition, and
it is the highest-value training signal (it teaches "it depends, here's when" instead of over-asserting).

## Tiers

- **confirmed** — reproduced across seeds (≥8) with a deterministic receipt.
- **confirmed-boundary / confirmed-negative** — a reproduced limit or failure mode (equally load-bearing).
- **asserted** — a synthesis/pattern across confirmed results; not itself directly tested.
- **candidate** — one clean run, not yet multi-seed-confirmed.
- **refuted** — a prior claim the sandbox could not reproduce (kept, marked, never silently deleted).
- **speculative** — exploratory; lives in `research-notes/`, must not leak into confirmed reasoning.

## Current roster (2026-07-10)

| word | tier | feeds | one-line |
|---|---|---|---|
| carrying-capacity, predation, thermal-death, selection | confirmed | (base ecology) | validated exp0 primitives |
| `civic-floor` → resilience | confirmed | `ucf.md` | a floor raises whole-system resilience under shifting pressure |
| — its boundary | confirmed-boundary | `ucf.md` | context-dependent: marginal/harmful under cyclical drain (overshoot) |
| `anticipation` | confirmed (bounded) | Civic Tome | foresight emerges under global/predictable pressure (5/5); fails local (supersedes 89.2%) |
| `metabolic-thrift` | confirmed | (seasons) | cyclical stress selects cheap regulation; active torpor is purged |
| `targeted-floor` → stable-adaptive | confirmed | `ucf.md` | a minority-only floor keeps an adaptive adversary from specializing (reproduces Exp 9) |
| `unconditional-floor → stasis`; `no-floor → gamble` | confirmed-negatives | `ucf.md` | uncond = life-support/stasis (adapt maxes 10/10); none = extinct 3/10 |
| `founding-diversity` | confirmed | Diversity Before Crisis | diversity must precede the crisis; a floor can't reliably create it late (Exp 8b) |
| `entrainment` | confirmed | **needs a home** → governance cadence | internal clock locks to & tracks a predictable rhythm; nature+nurture |
| `adaptation-speed-limit` | confirmed | `tokens_astris_auctor.md` | collapse is the adversary/agent speed *ratio*, not resources (Exp 5) |
| `reintegration-over-exclusion` | confirmed | `penumbra_accord.md` | restorative (repair+reintegrate) beats punitive (exclude); harm must be addressed |
| `transmission-trades-diversity` | confirmed | Civic Tome (Precedents) | precedent-following is bounded/double-edged; coordinates but homogenizes (resolves Exp 8's 2/4) |
| `niche-maintains-diversity` | confirmed | Diversity / predictability law | a persistent spatial niche maintains diversity a uniform world converges away (amphiphile kernel) |
| predictability law | asserted (spine + diversify-half supported) | `SYNTHESIS.md` | predictable→converge / unpredictable-with-a-niche→diversify |
| unpredictability is structure-dependent | asserted | — | variance helps only with a differentially-exposed reservoir |
| `regularity-buffers-uniform-stress` | candidate | — | under uniform stress, regular timing beats random (14-seed monotonic) |
| `metronome-vs-reprieve` | refuted (general) → bounded | — | predictable-relentless deadlier than random ONLY under differential stress |
| `seasonal-floor`; "89.2% anticipatory shielding" | refuted | — | floor not a clean positive under cyclical drain; the 89.2% never held |

**~14 confirmed headline words** toward the ≥20 goal. Full schema entries (with all six fields) live in
`CONSTELLA_TO_EXPERIMENTS.md`; the constitution's copy is the evidence bridge. This table is the index +
the dataset's row shape.
