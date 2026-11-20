# Classic port burndown

What is still to implement for a like-for-like Rust reimplementation of
Frontier: First Encounters, as of 2026-11-18 (after the accepted-95 parity closures, see [ACCEPTED_PARITY.md](ACCEPTED_PARITY.md)). This checkout is Classic only: the Modern GPU engine, the FFED3D integration, `assets.bin` art
packs and the Enhanced renderer and graphics-pack plans are out of scope and have
been removed. (The launcher was rebuilt as a Classic launcher.)

The source of truth is [`dashboard/burndown.js`](../dashboard/burndown.js)
(open `dashboard/index.html` for the chart). This page is a summary of it. Update
the dashboard first, then regenerate or edit this page to match.

## Summary

**35 of 230 work items are open: 25 in progress, 10 not started.** The foundation,
model-engine, visible-slice and audio milestones are complete (45 items).

| Area | Open | In progress | Not started | Done / total |
|---|---:|---:|---:|---:|
| Parity blockers (2026-10-26 audit) | 5 | 5 | 0 | 19/24 |
| Flight core | 2 | 1 | 1 | 12/14 |
| Galaxy and systems | 2 | 2 | 0 | 11/13 |
| Gameplay systems | 9 | 8 | 1 | 34/43 |
| Original drawing: models | 5 | 2 | 3 | 14/19 |
| Original drawing: screens and effects | 6 | 6 | 0 | 25/31 |
| Content import | 1 | 1 | 0 | 6/7 |
| Saves | 2 | 0 | 2 | 23/25 |
| Classic split cleanup | 0 | 0 | 0 | 2/2 |
| Release and sign-off | 3 | 0 | 3 | 4/7 |
| **Total** | **35** | **25** | **10** | **195 / 230** |

## Critical path

1. **Parity blockers**: the P0 items first (event units, takeoff handler,
   contact rules, Hope start, player explosion), then the P1 items.
   Most of the flight, world and gameplay work depends on them.
2. **`flight-1`, the object system and agent records** (`cstruct`/`gmstruct`).
   Not started, and much of the remaining flight and gameplay work builds on it.
3. **Flight core, galaxy and gameplay systems**, in the order given in
   [STATUS.md](STATUS.md#work-list-to-finish-the-game--2026-11-04-handoff).
4. **Original drawing**: the planet surface quads, the whole-frame pixel
   oracle, sprites and particles, and the last 17 model slots.
5. **Saves, content and release**: legacy commander import, CD 1.01
   verification, full playthrough checkpoints and the owner's sign-off.

## Open items by area

`[~]` is in progress and `[ ]` is not started.

### Parity blockers (2026-10-26 audit) (5 open)

- [~] `parity-08` **P0** Restore the complete third-person player explosion before GAME OVER
- [x] `parity-15` **P1** Replace NPC speed caps and simplified flight state transitions (accepted at about 80%, see ACCEPTED_PARITY.md)
- [~] `parity-20` **P1** Replay market, shipyard, BBS and daily mission events using source state
- [~] `parity-21` **P1** Drive all explosion, debris and hit effects from source object states
- [~] `parity-22` **P1** Replay missile, ECM, escape and legal outcomes in the complete event loop
- [x] `parity-23` **P1** Remove silent fallback to the synthetic playable from normal starts/loads
- [~] `parity-24` **P1** Use the same source new-game setup after death and on every resume path

### Flight core (2 open)

- [ ] `flight-1` Object system and agent records (cstruct, gmstruct)
- [~] `flight-5` Original input processing, key accumulation and remapping

### Galaxy and systems (2 open)

- [~] `world-8` Navigation: system and galaxy maps
- [x] `world-9` Lighting by the local star (ships, stations, lift) and day/night — F1481's source light sign feeds all three object paths; F1535 star-class tables (including Ross 154's flagged class) and the planet terminator are verified with focused cache-backed regressions.
- [~] `world-13` Source model scale and near-surface radius/frame transitions

### Gameplay systems (9 open)

- [~] `gameplay-5` Missiles and countermeasures
- [~] `gameplay-6` Agents: AI behaviour (cagent)
- [~] `gameplay-7` Police, pirates and other encounters
- [ ] `gameplay-8` Combat scenario replay against the reference
- [~] `gameplay-9` Bulletin board and missions (cbbs)
- [~] `gameplay-10` Journal and mission state
- [~] `gameplay-12` Communications and messages
- [~] `gameplay-18` NPC flight/combat AI (DATA_006258), ship fire, police/wanted and jump persistence
- [x] `gameplay-26` Saker Mk III default arm, F2 loadout and start records — source comparisons cover start types 0/1/2/3/5; cache-backed default Hope, Ross 154 and F2 hold regressions pass.
- [x] `gameplay-38` Runtime module clock, source F3/F4 and landed/flight event routing — 5 runtime and 3 player-event regressions pass; one original-capture comparison remains fixture-gated.
- [x] `gameplay-40` Starport pad descent F741-F746/F652 with 0x59 geometry import — pad landing runtime and model 0x59 collision geometry tests pass with the local content cache.
- [~] `gameplay-41` Mission-script engine (F41/F31): script import, journals and scripted missions

### Original drawing: models (5 open)

- [~] `v0.48-7` F1834 / F1835 quad emission (with F1868-F1873)
- [ ] `v0.48-8` Surface quad oracle at a planet-scale distance
- [ ] `v0.48-14` Pixel-level whole-frame oracle (framebuffer mode, station and three ships)
- [ ] `v0.48-16` Sprites, particles and circles (F1609-F1612)
- [~] `v0.48-17` Remaining 17 of 466 model slots and the CD 1.01 census

### Original drawing: screens and effects (5 open)

- [~] `ui-5` Menu and text rendering for every screen
- [x] `ui-7` Intro and new-game flow — owner authorized closure of the remaining user QA gate; recorded intro checks were already passing. Direct `L`/load-picker behavior is outside this item.
- [~] `ui-10` Original palette effects and screen transitions
- [~] `ui-11` Far and near transitions, clipping and planet approach fidelity
- [~] `ui-12` Hit particles, explosions and debris (F939/F942/F926)
- [x] `ui-25` Galaxy map F7 description page: F850/F851 body diagram, F830/F1347 string templates, F842 text — Gateway's frame matches the original capture; 5 unit tests and 2 cache-backed tests pass.
- [x] `ui-26` Galaxy map F9/F10 economy and people pages (F843) — Gateway's two frames match the original captures; the cache-backed test checks every good and line.
- [x] `ui-27` Galaxy map body click info: mass, temperature, starports, orbit (F843 0x80-0xbc) — Gateway 5's facts (299.02 Earth masses, -99 C, 104.7 years, 21.050 A.U.) match the original capture.
- [x] `ui-28` Galaxy map F8 picture mode: the star-name sky (F844-F849) — all nine names land where the original capture has them; the sky is the final of F844's three passes.
- [~] `ui-29` 3D system map page (module 5, F963-F982/F985): orbit curves, body labels, the 'Time point' line, mouse turn and zoom, F5-F12 into the galaxy pages — not created; the port has only the Tab body list and the `n` places list.
- [~] `ui-30` System map object click (F983/F986): target or autopilot destination for a clicked planet, moon or station.
- [x] `ui-24` Display filters and scanlines — owner authorized closure of the remaining user QA gate; focused implementation/settings checks were already passing.

### Content import (1 open)

- [~] `content-2` Message strings, journal and remaining fonts
- [x] `content-4` Optional CD media inventory and lossless packaging (videos and stills) — 323 AVI + 323 BMP, verified FMVs manifest; see [CD media](notes/cd-media.md)

### Saves (2 open)

Completed 2026-11-15: `persist-6` through `persist-25` are twenty bounded native
resume fixes under PAR-18; see [persistence](notes/persistence.md#21-live-resume-records-2026-11-15).
The twenty additions are complete, so the remaining count is unchanged. The summary
also includes the previously added, still-open display-filter task `ui-24`.

- [ ] `persist-1` Legacy commander import (read-only)
- [ ] `persist-4` Imported saves replayed against the reference

### Release and sign-off (3 open)

- [ ] `release-1` Full playthrough checkpoints against the reference
- [ ] `release-6` Choose the Rust code licence
- [ ] `release-7` Like-for-like parity sign-off by the owner
