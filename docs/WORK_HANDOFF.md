# Work handoff

## Current authorization

Owner requested completion work for existing item `ui-24` only. Preserve the mixed checkout; do not resume save/load work, change another item, commit or push. Keep `ui-24` open until its manual appearance QA gate is satisfied.
## Verified baseline — 2026-11-15

- Dashboard: 224 total, 147 done, **77 remaining**. PAR-23 is closed; no other item changed status in this handoff.
- Mixed checkout remains uncommitted. Preserve unrelated edits. `cargo clippy --locked -p ffe-rs --bin ffe-rs -- -D warnings` passes.
- PAR-23 evidence: six `source_boot` cases and five source-start comparisons pass; malformed cache/ship records and unsupported saves error; default debug/release three-frame startup smoke exits 0. Details are in its audit entry and dashboard history.

## Active contract — dashboard `ui-24`

**Gap:** xBRZ/Scale2x filters, optional CRT scanlines and settings UI/persistence are implemented; dashboard history says manual QA remains. Verify selection, composed output routing, scanline behavior and persisted settings; fix only concrete focused-test failures.

**Acceptance:** available filter choices select and persist; optional scanlines and intensity reach the presenter and persist; focused filter and settings tests pass. The focused checks pass (12 filter tests, display serialization, versioned config persistence, Display Options interaction). Owner appearance QA is the remaining gate. Keep dashboard totals fixed; `ui-24` stays open until that gate is reported complete.

**Relevant files:** `crates/ffe-rs/src/filters/{xbrz,scale2x,scanlines,mod}.rs`, `crates/ffe-rs/src/display.rs`, `crates/ffe-rs/src/prefs.rs`, `crates/ffe-rs/src/main.rs`, `crates/ffe-rs/tests/prefs.rs`, `dashboard/burndown.js`, `docs/BURNDOWN.md`.

**Smallest existing focused checks:**

```powershell
cargo test --locked -p ffe-rs --lib filters::
cargo test --locked -p ffe-rs --test prefs
```

Do not begin any other backlog item.
## Readiness queue — not additional active work

| Existing ID | Readiness / unresolved gate |
|---|---|
| `gameplay-26` | Source starts 0/1/2/3/5 matched original fields. Review F2/hold behavior and affected old tests after correcting Ross 154 capacity. |
| `ui-24` | Implementation and focused checks pass; owner appearance QA remains. |
| `ui-7` | Intro oracle passed 76 cases. New source start choices and native load picker need UI/flow validation. |
| `flight-5` | Existing keyboard replay passed 730 frames; review remaining mouse/remap/module gates before closure. |
| `gameplay-38` | Mode-independent player events extracted; module routing/regressions need completion-gate checks. |
| `gameplay-34` | Servicing matches 96 original cases. F952 cargo gate and F953/F950 RNG/hit routing remain unfinished: NOT ready to close. |
| `gameplay-dust` | Needs source lifecycle/view-space culling audit; do not treat existing rendering tests as full source parity. |
| `gameplay-12` | Scope/remaining comms and message formatting must be established before edits. |
| `release-4` | Packaging/audit acceptance not established; no implementation yet. License and owner sign-off are separate open items. |

## Unfinished work to preserve

- Startup changes include source starts 2/3, corrected type-1 hold/general bits,
  source key 4 load picker, source key 5 start, supporting frames on pad loads,
  and per-commander UI/script reset. These extend beyond PAR-23; do not bundle
  their completion claims into that item.
- Events changes add persisted F934 servicing countdown and exact RNG/counter
  behavior, correct F933 to 29 attempts, and move shared player events outside
  the flight-only branch. `tests/player_events.rs`: four cases passed, including
  96 original F934 dispatches. Full old-suite compatibility is unverified.
- Earlier persistence work added `persist-6` through `persist-25` as completed
  subtasks. Those additions did not reduce the original backlog. Do not count
  them again or rewrite history to conceal the scope mistake.
- Detailed local investigation: `.cache/backlog-78-to-68.md` (historical and
  partially stale). Original captures: `.cache/start-records.jsonl` and
  `.cache/service-records.jsonl`; reference binary under `.cache/jjffe-reference`.
  Keep originals/captures/binaries out of commits and external services.

## Handoff discipline

Keep one active contract. Replace its progress notes after each meaningful
result. Do not expand this document into another port log. When switching chats,
provide this file and the selected ID rather than the entire conversation.
