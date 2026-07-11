# Audit / polish backlog — ALife ↔ Constella ↔ FAITHH

*A running list of places worth a second look. **Lightweight by design:** act on high-reward items,
skip or defer low-reward ones. Not a process — a scratchpad so nothing quietly drifts. Add rows as we
notice things; don't over-invest.*

Legend — **status:** open / done / skipped (low reward) · **reward:** ⭐ (nice) → ⭐⭐⭐ (load-bearing)

## Word → mechanism mappings (does each earned word match the *actual* Constella doc?)
| location | issue | status | reward |
|---|---|---|---|
| `Penumbra_Accord.md` vs evidence-bridge Penumbra section | **RESOLVED.** Doc is *restorative justice*, not the "strategic-dissolution" the bridge attached. Built the real experiment (C2, `pen` mode) → confirmed `reintegration-over-exclusion` (restorative 10/10 vs punitive 7/10, 3× the diversity). Ledger updated; bridge Penumbra section still needs the swap (next sync PR). | done (bridge sync pending) | ⭐⭐⭐ |
| `tokens_astris_auctor.md` vs `adaptation-speed-limit` (C1) | **VERIFIED — sound, already partly applied.** The Decay section *explicitly* cites the ALife sandbox ("unconditionally protected incumbents freeze the system and kill adaptation") — our `unconditional-floor→stasis` finding already removed the "Legacy Astris: no decay" carve-out. C1 *adds* a refinement the doc lacks: the decay RATE (currently 2%/week, set without justification) should be **bounded by participant adaptation speed**. Optional follow-up: note that in `tokens_astris_auctor.md`. | done (opt. refinement) | ⭐⭐ |
| `civic_tome.md` vs `anticipation` | **SOFT MATCH (defensible).** Tome = versioned record of protocols/precedents/rulings; "precedents guide future decisions" ≈ pattern-memory-for-anticipation. Interpretive but fair. | done | ⭐ |
| `civic_tome.md` vs `entrainment` | **MISMATCH (2nd catch).** `entrainment` = rhythmic phase-locking to a *periodic* environment; the Tome is a records system, not a cadence. Entrainment has **no home** in the current docs — it points to a *missing* mechanism: **governance cadence / rhythm** (scheduled cycles a community phase-locks to).  **HOME BUILT (2026-07-10):** `governance_cadence.md` (constella PR #67) — the missing cadence mechanism entrainment pointed to. | open | ⭐⭐ |
| `core_framework.md` "three-mechanism system" | **CHECKED — no action.** The claim lives only in the evidence bridge, not `core_framework.md` (which frames UCF as "voluntary baseline dignity" = the *dignity* floor — consistent with the dignity-vs-diversity split). No boundary caveat needed there. | done | — |
| `ucf.md` dignity-vs-diversity split | **DONE (already in the doc — audit note was stale).** `ucf.md` has a "Two floors under one name" section separating the Dignity Floor (unconditional-by-right, funded & bounded) from the Diversity-Maintenance Pulse (targeted, threshold-activated), citing Exp 6/9 + the Rust 787/43 result. *Optional add:* the seasons boundary (the diversity-pulse is context-dependent — helps vs adversity that removes variation, marginal vs cyclical drain that starves a crowd). | done (opt. refinement) | ⭐⭐⭐ |
| `core_framework.md` "three-mechanism system" | Still stated as clean; the seasons work added a **boundary** (floor is context-dependent). Does the framing need the caveat? | open | ⭐⭐ |

## Stale claims / figures (attestation integrity)
| location | issue | status | reward |
|---|---|---|---|
| Exp 3 "89.2%" in the evidence bridge | REFUTED → corrected in place (PR #62). | done | ⭐⭐⭐ |
| Other hard numbers in governance docs (compliance_overlay, founding_hypothesis, PoL, map_of_intent) | **DONE — CLEAN.** Swept 2026-07-10: no unbacked empirical figures like the 89.2%. The proof_of_life_consensus percentages are *design weights* (scoring formula, sum 100%), not empirical claims. Attestation integrity holds. | done | ⭐⭐ |

## Sandbox / receipts
| location | issue | status | reward |
|---|---|---|---|
| Dormant ops `signal` / `toxin` (`SIGNAL_ACTIVE`/`TOXIN_ACTIVE=false`, "exp5") | Built but never exercised; no Rust receipt. The Penumbra experiment will finally use `toxin`. Is "exp5" documented anywhere? | open | ⭐ |
| Confirmed words without a linked reproducible receipt command | Spot-check each ledger entry has a runnable `alife-core …` line. | open | ⭐ |
| Penumbra **gamer-lifecycle** claim (Exp 9) is Python-asserted, un-received | Tried a Rust receipt (b2 floor-dependent fraction, 2026-07-11) — **did NOT reproduce** (4/8, no correlation with stabilization). The b2 mechanic doesn't replicate Exp 9's NAKED/DEFENDER + founding-window dynamic. A dedicated Exp-8b/9 replica would be needed. Until then the bridge's gamer-lifecycle text should stay *asserted*, not claimed as Rust-confirmed. | open (bounded) | ⭐⭐ |

## Infrastructure / process
| location | issue | status | reward |
|---|---|---|---|
| Word-schema not yet formalized | **DONE.** Created `alife/WORD_SCHEMA.md` — the canonical shape (word/claim/tier/holds-when/fails-when/receipt/feeds) + tier defs + roster. Shared by the ledger, the constitution bridge, and the eventual training dataset. | done | ⭐⭐ |
| Two ledgers to keep in sync | `alife/CONSTELLA_TO_EXPERIMENTS.md` (working) + `constella/.../alife_evidence_mapping.md` (constitution). Manual sync each batch (PRs #62, #63). Fine for now; revisit if it gets heavy. | open | ⭐ |
