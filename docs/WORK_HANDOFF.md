# Work handoff

## Closed 2026-11-18 — dashboard `gameplay-dust`

Closed on the executed-original comparison in `docs/ACCEPTED_PARITY.md` (153 port dust pixels against 151). The contract below is kept as history; the three gaps left (Milky Way band, day-tick ageing, crosshair pixels) are recorded there.

## Former contract — dashboard `gameplay-dust`

**Gap:** owner sees a dense grey dust cluster after climbing away from a planet, absent in the original comparison. Recent absolute-chain sizing may expose a separate dust detail/lifecycle mismatch.

**Acceptance:** reproduce ascent, identify DOS-backed cause, correct cloud placement/size/count or lifecycle without arbitrary density suppression, preserve dust preference and motion; verify matched original/port behavior before closure.

**Files:** `crates/ffe-core/src/{dust,model_execute,model_commands}.rs`, `crates/ffe-rs/src/{main,screen,navigation}.rs`, `crates/ffe-rs/tests/effects_view.rs`.

**Smallest verification:** core dust tests and local-cache `effects_view` dust regressions, then ascent capture and DOS comparison. Dashboard status/totals unchanged. Background parity remains parked and unverified.
**Progress:** removed main.rs's port-only 75 km dust visibility cutoff (including spherical near-body check). F1035 already creates clouds above 20 km; hiding them until 75 km can reveal accumulated clouds together. DOS 1.02 file 0x3277f compares altitude at 0x642218 to 0x1312d00; JJFFE F1035 agrees. Preference still controls visibility. This is a source-backed correction, not a reproduced explanation of every grey pixel in the owner's image. Owner confirms onset after climbing away. Twelve core dust tests and five ignored cache-backed effects_view dust tests pass; cargo check for game binary passes. Strict binary clippy fails on unrelated main.rs type_complexity near line 8122; rustfmt --check reports existing unrelated formatting near line 507. Logs: `.cache/parity07/dust-{core,view,build,clippy,format}.log`. No dashboard status/totals changed. Next: matched ascent capture with dust on/off, confirm whether cluster persists after removing cutoff, and audit F1680 hidden/cull feedback (particles currently retain hidden=false).


## Parked contract — dashboard `parity-07` (owner flight-background correction)

**Gap:** model 315 currently uses unverified depth/orientation/placement/light; geometry reaching a frame and the unmatched final capture do not establish parity.

**Acceptance:** DOS FFE 1.02-derived call arguments and bitmap 53–58 masking; reliable front-view nebula and source star chains; matched state/view/options/logical-resolution captures establishing orientation, extent, palette, brightness, star count and starburst shape; preserve motion, visibility settings and haze. Keep open until every visual criterion is verified.

**Relevant files:** `crates/ffe-rs/src/{screen,model_viewer}.rs`, renderer texture paths, existing reference harness under `tools/reference`, `.cache/reference-jjffe/ffe.asm`, `game/FIRSTENC.EXE`, `.cache/parity07`.

**Smallest verification:** `cargo test --locked -p ffe-rs flight_background_model315_reaches_frame` is geometry regression only; completion additionally requires DOS source comparison and matched original/port flight captures.

**Verified correction:** F1766 absolute star-chain sizes now bypass depth division and the instance-depth gate. DOS 1.02 file 0x51773 masks with 0x7fff and jumps directly to 0x517cc, bypassing F1666 at 0x517c1; JJFFE agrees. The port previously divided these absolute sizes, collapsing model 316's packed 0x0140 (two pixels) to zero at distance. Changed `crates/ffe-core/src/model_execute.rs` and its existing integration test. All 33 model_execute tests pass; strict clippy for that test passes; ignored flight_background_model315_reaches_frame passes with local data.bin. The latter proves frame integration only, not visual parity. Latest synthetic capture: `.cache/parity07/flight-background-source-transform.ppm` (unmatched).

**Second verified correction:** face emission now honors F1762's normal-0 colour write. Model 315 resets normal 0 to black, but faces previously kept F1480's initial shade, adding the system tint to the nebula. DOS file 0x51475 tests bit 0x80 and 0x51489 loads normal-table base 0x653088 for F1691 at 0x5148f; JJFFE F1762 agrees. Added emission-order regression; all 34 model_execute tests, ignored flight background test, strict core test clippy and formatting pass. Same synthetic port scene before/after saved as `.cache/parity07/nebula-colour-{before,after}.{ppm,png}`: visible shift from pale pink to dark blue-violet. These are port-to-port frames, not matched DOS parity evidence. No depth/orientation change in this step.

**Unresolved:** nebula transform/light/palette and matched original/port UI capture. DOS F1775 at 0x51cac, transpose F1665 at 0x4ac60, F1682 at 0x4c140 and F1683 at 0x4c164 confirm direct z=0x200. Prior direct-transform experiment was reverted; production still uses unverified 0x2000/star orientation. Investigation trace `.cache/parity07/execution.log` shows model scale 10 and child 316 scale 8; missing geometry was not proof that the DOS depth was wrong. Next inspect model 315's camera parent frame and texture 53–58 descriptors/masking, then extend the existing capture path. No temporary tracing remains in the renderer. Dashboard unchanged; parity-07 stays open.

## Current authorization

Work through the existing police/pirate parity items in dependency order, one dashboard ID at a time. Preserve mixed-checkout edits; do not add dashboard tasks, commit, push or spawn bots.

## Parked contract — dashboard `parity-17`

**Gap:** the shared RNG pair is implemented and existing source-backed cases match F622 plus F688 for default and Ross 154 starts. The full draw ledger remains incomplete for F633 daily draws and later consumers, including object-event and rendering dependencies.

**Acceptance:** record a baseline draw ledger including object-event and rendering dependencies; match both source words and downstream outcomes. Optional port presentation must not silently choose a different gameplay seed.

**Relevant files:** `crates/ffe-core/src/{npc,spawn}.rs`, `crates/ffe-rs/src/navigation.rs`, `tools/reference/{headless.c,README.md}`, `docs/audits/gameplay-parity-work-items.json`.

**Smallest verification:** use the existing `bbs_reference` cases for F622/F688, then compare a same-seed executed-source `FFE_RNG_TRACE` against the port through F633 and downstream consumers. The current checkout lacks the complete JJFFE smoke-build inputs and NASM executable for new source traces.

**Progress:** `parity-15` depends on this item. Existing ignored `bbs_reference` cases record the default F622+F688 1,876-draw trace and Ross 154 entry/exit states; they supersede the older claim that F688 diverges at draw 1712. `parity-16` is already done on the dashboard; its remaining F702 placement/F238 police-response work is carried by `parity-15`. This turn corrected the contract to match current evidence; no source code changed.

## Parked contract — dashboard `parity-15`

**Gap:** source-like physics is implemented for ships with statistics, but per-state NPC traces remain unmatched; the stats-less speed-cap fallback remains. Police response is currently reconstructed from wanted/alert bits, and F702 system-wide placement remains carried from `parity-16`.

**Acceptance:** trace traders, escorts, pirates, police and heavy freighters under the same objects/seed; match per-state target, thrust, turn, velocity and event time at 1x and accelerated time. Keep the full PAR-15 acceptance unchanged.

**Relevant files:** `crates/ffe-core/src/npc.rs`, `crates/ffe-rs/src/navigation.rs`, `tools/reference/{headless.c,verify-npc.py}`, `.cache/reference-jjffe/ffe.asm`, `docs/audits/gameplay-parity-work-items.json`.

**Smallest verification:** same-seed source/port `--npc-trace` comparison at 1x and accelerated time, including police response and F702 placements.

**Progress:** source F238 uses the player's law-response/count fields, calls F239 to allocate and construct police objects, and consumes shared RNG; the port instead provokes all existing police/military from `Legal::wanted`. The exact F238 path is a multi-state behavior change. No source edit started; preserve PAR-15 as in progress until its trace gate is available and full acceptance passes.

## Latest completion — dashboard `ui-7`

**Delivered:** closed the existing Intro and new-game flow item. Direct `L` and the load-picker flow are outside this dashboard item's scope.

**Evidence:** existing source intro checks cover 76 cases; nine `front_end` tests and the intro-reference compile are recorded as passing. The owner authorized closure of the remaining UI QA gate. Live key delivery and a matched-frame capture were not verified in this session; no direct-load behavior is claimed.

**Dashboard:** 224 total, 154 done, 70 open (58 in progress, 12 todo).

## Latest completion — dashboard `content-1`

**Delivered:** verified and closed the existing ship, equipment and economy table import item. Mission-cargo handling and F641 board generation remain in their gameplay items.

**Evidence:** `cargo test --locked -p ffe-core --test market` (14 passed); absolute-cache `ship_names -- --ignored` (2 passed, all 466 model slots and 58 hulls); `cargo test --locked -p ffe-core --lib equipment::tests` (16 passed); `cargo test --locked -p ffe-rs --test fitting` (6 passed). Source records document the DOS equipment-table byte comparison and 8,192 executed-original market cases.

**Changed files:** `dashboard/burndown.js`, `docs/BURNDOWN.md`, `docs/STATUS.md`, `docs/WORK_HANDOFF.md`, `docs/notes/market.md`.

**Dashboard:** 224 total, 153 done, 71 open (59 in progress, 12 todo).

## Parked contract — existing `audio-3` launcher music bundle extension

**Gap:** audio is rendered by the game process into per-track WAV cache files. The owner wants generation moved to launcher first-run after a valid game folder is set, with one `music.bin` bundle gating regeneration.

**Acceptance:** launcher generates all 17 GeneralUser GS renders only when root `music.bin` is missing; writes it atomically; the game loads its songs from that bundle; changing the selected game folder invalidates it; existing bundle is reused; original game files and effects stay unchanged.

**Relevant files:** `crates/ffe-launcher/Cargo.toml`, `crates/ffe-launcher/src/{main,checks,config}.rs`, `crates/ffe-audio/src/{render,audio}.rs`, `crates/ffe-rs/src/sound.rs`, `crates/ffe-audio/tests/hmp_assets.rs`, `.gitignore`, `Cargo.lock`.

**Smallest verification:** audio bundle round-trip/cache-gate tests, launcher check tests, `cargo check --locked -p ffe-launcher` and `cargo check --locked -p ffe-rs --bin ffe-rs`.

**Progress:** implemented. 29 audio unit tests pass; the optimized local 17-song integration renders and packs `music.bin`, reloads every track and confirms reuse; launcher source-folder invalidation tests pass (2). Launcher/game release build passes in `target/music-bundle-test`. Linux device output remains unverified on this Windows host; OxiSynth release notices/relinking compliance remain before distribution.

## Parked contract — dashboard `content-2`

**Gap:** message arrays, journal archive/stories, viewer and the 256-byte `DATA_007648` width asset are present. F1392 model text uses F1398 width `0x23`; subscription status, F76 `Sl`/`Oo`/`Re`/`Ri`/`Rf`, and font control anchors are implemented. Other F76 macro groups and text source-oracle coverage remain open.

**Acceptance:** complete the existing message-string, journal and font scope against the source; verify the archive, viewer/macros, subscription messages and remaining font behavior with focused tests and applicable source comparisons. Keep open while any of those scoped behaviors remain unresolved.

**Relevant files:** `crates/ffe-core/src/{journal,journal_library,message_strings,script/brackets}.rs`, `crates/ffe-rs/src/{journal_page,model_viewer,scripts}.rs`, `crates/ffe-rs/tests/{message_strings,font_reference,station_view}.rs`, `docs/notes/{text,journal}.md`.

**Smallest verification:** run `cargo test --locked -p ffe-core script::brackets::` and `cargo test --locked -p ffe-rs --lib journal_page::`; then use the local content cache for ship-library and text/font source-vector checks when available.

**Current task contract:** `Cs` now copies the current-system F860 name after F75 consumes its following source byte; the archive regression shows `Gatewayand`, preserving the source spacing behavior. Remaining `Sy`/`Ci` enter F76's F858 generated-system path, try up to five generated codes, fall back to the current system, and use F860 naming plus F461/F462 in the alternate path. F858 has no equivalent in the current Rust runtime, whose `Texts.world` carries only the current system and known destinations. Do not substitute a nearby destination or start a new system-generation subsystem under this item without revising scope/estimate. `Sy` appears 67 times in the imported archive. Relevant files: `crates/ffe-core/src/{script/brackets,names}.rs`, `crates/ffe-rs/src/{navigation,scripts}.rs`, `crates/ffe-rs/tests/station_view.rs`, `docs/notes/journal.md`. Smallest verification: focused bracket tests and ignored archive census using `.cache/content-census.bin`. Keep full `content-2` acceptance unchanged. `Si` still reads `DATA_008861+0x124`; `Flight` stores only the ship model index, and native `Commander` saves have no separate ship-object string. Do not substitute `Sh`'s model display name.

**Progress:** archived JJFFE `ffe.asm` confirms F1392 calls F1398 with width `0x23` (35) for `0x3022`. `ModelViewer` reads the imported signed-width table and applies the existing F1398 wrapper. The archive draws F76 subscription strings `0xa605`/`0xa606` at x=200, colour 0xf, with the source period/expiry count. F76 `Sl`/`Oo` use F75/F921 to choose alternatives (with `Oo` recursively expanding its choice); `Re`/`Ri`/`Rf` map source score thresholds to imported rank words; `Sh` resolves the player's ship model through its statistics name code. Cache-backed tests confirm subscription strings, all Elite/Federal rank words and all 13 Imperial words; first issues across five journals expand without bracket fallbacks. The ignored cache-backed census covers all 671 imported journal articles and counts 616 bracket prefixes. It confirms archive uses of `Ci` (10), `Cr` (13), `Cs` (11), `Sy` (67) and `Zz` (8). Its sole `[Da` occurrence is not an F76 macro: JJFFE's `DATA_102307` table contains no `Da` entry (`ffedat.asm:3033-3034`). Source trace in `FUNC_000076` confirms `Cr` and `Co` both call F791 with company-name kind 12, matching the existing code; a regression now covers `Cr`. `Zz` is source-ported: it resets its F74-selected lazy-random-word slots and emits no text. `Cs` is source-ported with a cache-backed real-article regression; `Sy`/`Ci` remain unresolved because F858's generated-system search path uses F921/F853 and has no equivalent in current runtime data. Preserve the unchanged acceptance scope. The three font newline anchors import as 58/38/20 and F1747 uses them for control-code line moves; cached and synthetic tests pass. `Si`'s source object string has no equivalent in the current `Flight` or native save model; implementing it requires identifying and persisting the source value, so it remains open. This turn's full-archive prefix census passed with the local cache; it also exposed and fixed a `screen.rs` borrow mismatch that prevented compiling the test target. Model-text source-oracle and other macro behavior remain open. Dashboard unchanged: 224 total, 153 done, 71 open.

## Parked contract — dashboard `ui-5`

**Gap:** the owner reports Save/Load/Delete appear grey. Preference tests and a captured frame show those rows enabled, but the live menu path has not been verified. This dashboard ID covers every screen's menus and text.

**Acceptance:** trace the real Escape/menu input path, verify Save/Load/Delete/New Game render active and route in applicable states, and retain the broad every-screen scope before closure.

**Relevant files:** `crates/ffe-rs/src/{main,prefs,screen}.rs`, `crates/ffe-rs/tests/prefs.rs`, launcher startup in `crates/ffe-launcher/src/main.rs`.

**Smallest verification:** `cargo test --locked -p ffe-rs --test prefs`, followed by a configured-launcher in-game check.

**Progress:** `cargo test --locked -p ffe-rs --test prefs` passes (13 tests). A bounded launcher capture and replayed Escape verify the main preference branch and active Load/Save/Delete rows; screenshot: `.cache/ui5-replay-route-bounded-20261116/start-0040.png`. Physical keyboard delivery and broad every-screen menu/text coverage remain unresolved. Resume after `content-2` or another item; do not close on the preference screen alone.

## Audio bundle implementation progress — `audio-3` scope extension

Launcher generation and game loading use a single root `music.bin`. `cargo test --locked -p ffe-audio --lib` passes (29); the local 17-track integration renders, packs, reloads and confirms reuse; launcher pack-invalidation tests pass (2); `cargo check --locked -p ffe-launcher` passes. The owner will verify the tracks in-game. Linux audio output still needs a Linux/ALSA build and device check. OxiSynth is LGPL-2.1; distribution notices/relinking compliance remain before shipping binaries. GeneralUser GS is bundled with its license; its author notes uncertain provenance for some samples.

## Parked contract — dashboard `parity-07`

**Gap:** owner-supplied in-game comparisons show that flight model 315 is often missing or off-axis, with too few visible starbursts and a redder nebula than the original. The flight adapter must preserve F1775's system orientation while mapping its camera-local placement into the renderer's view space. The intro also retains a separate nebula appearance gap in model 176's F959 path.

**Acceptance:** use model 315's source star chains and F1775 transform/light in flight; recover the original starbursts and nebula visibility/colour shown in owner captures. Compare source and port screenshots and preserve camera/world motion and palette behavior. Keep `parity-07` open until its broader Hope start/cockpit image scope is complete.

**Relevant files:** `crates/ffe-rs/src/screen.rs`, `crates/ffe-rs/src/model_viewer.rs`, `crates/ffe-rs/src/camera.rs`, `crates/ffe-rs/tests/intro_view.rs`, `crates/ffe-rs/tests/effects_view.rs`, `docs/notes/intro.md`, `docs/notes/dust.md`.

**Smallest verification:** inspect existing model 315/star-chain tests and the supplied original/reference capture path; then run focused intro and flight renderer tests with `.cache/parity07/data.bin`.

**Progress (2026-11-16):** JJFFE F1775 copies the active matrix, writes local z `0x200`, and calls F1682 for model `0x13b`. With the real new-game station orientation, the old raw `0x200` view-space placement yielded no star spans; a view-space projection depth of `0x2000`, the system star orientation, and camera-forward placement now produce a centered vertical blue-violet nebula and model 315 star-chain spans in the captured 320x200 flight frame (`.cache/parity07/flight-background-final.png`). The background uses the viewer's default light instead of local-star lighting, correcting the red tint. `effects_view::flight_background_model315_reaches_frame` passes with `.cache/parity07/data.bin`; it checks model textures and star spans reach the live flight frame. Owner screenshots still show a denser, more visible starburst field than the port capture, so visual parity is not complete. The intro model 176 nebula and broader Hope start/cockpit image scope also remain; keep `parity-07` open.

## Verified baseline — 2026-11-16

- Dashboard after latest completion: 224 total, 153 done, **71 remaining** (59 in progress, 12 not started). PAR-23, `ui-24`, `gameplay-26`, `gameplay-38`, `gameplay-40`, `world-9`, and `content-1` are closed; `release-4` remains in progress for audit scoping.
- Mixed checkout remains uncommitted. Preserve unrelated edits. `cargo clippy --locked -p ffe-rs --bin ffe-rs -- -D warnings` passes.
- PAR-23 evidence: six `source_boot` cases and five source-start comparisons pass; malformed cache/ship records and unsupported saves error; default debug/release three-frame startup smoke exits 0. Details are in its audit entry and dashboard history.


## Latest completion — dashboard `gameplay-26`

**Delivered:** completed the F998 start-record, Saker/Ross 154 hold, fitted-loadout and F2 review.

**Evidence:** existing executed-source comparisons cover start types 0/1/2/3/5. With isolated `.cache/parity07/data.bin`, the default Hope start, Ross 154 start, and F2/services hold tests each pass. Dashboard totals: 224 total, 151 done, 73 remaining; `gameplay-26` done.

**Changed files:** `dashboard/burndown.js`, `docs/BURNDOWN.md`, `docs/STATUS.md`, `docs/WORK_HANDOFF.md`.

**Next:** correct the stale space-dust deviation text and retain the source-renderer gate under `gameplay-dust`.

## Progress notes — existing open items in this batch

- `ui-7`: closed under the owner's QA authorization; live input delivery was inconclusive. Do not treat the port-only `L` shortcut or load-picker flow as part of this dashboard item.
- `ui-5`: focused Display Options corrections verified (`cargo test --locked -p ffe-rs --lib prefs`: 8 passed). Display Options menu row and page heading are white; regression coverage asserts no preference menu row uses yellow. `git diff --check` passes. Broad `ui-5` remains open; preserve those uncommitted changes.
- `flight-5`: 14 input unit tests and 3 controls integration tests pass, but `settings.rs` and persistence notes confirm `bind.*` remapping is reserved and unused. The missing remap UI is outside a quick correction; no code changed and the item stays in progress.
- `gameplay-12`: with `.cache/parity07/data.bin`, `station_view::mission_offer_journal_and_comms_frames` passes. The wider item remains in progress because mission notes still identify military offer constructors and message behavior needing source-state coverage.
- `gameplay-41`: all 8 ignored `mission_scripts` tests pass with the absolute `.cache/parity07/data.bin` path. The full scope remains open for F669 event ordering and unresolved journal controls/timing documented in `docs/notes/mission-scripts.md`.
- `ui-7`: the original intro capture and port captures remain at different animation moments; they were retained locally as QA artifacts, not claimed as a source-frame comparison.
- `gameplay-dust`: 12 core dust tests and five cache-backed `effects_view` render regressions pass. The required source-backed view-space/culling comparison remains unavailable; leave in progress.
- `gameplay-34`: its three non-fixture player-event tests pass; the 96-case source comparison requires missing `FFE_SERVICE_RECORDS`. Source audit still lists missing combat-score state and F952/F953/F950 routes; stays in progress.
- `release-4`: audit scoping started; no release workflow or package outputs were found, and packaging acceptance is not defined. Dashboard now marks it in progress; no completion.
- `gameplay-39`: two autopilot control tests, four cache-backed runtime tests and the focused cancel test pass; keep in progress because linked PAR-11 still lacks avoidance, moving-target pursuit, rotating-parent guidance and event-cadence replay.

## Partial progress — dashboard `ui-5`

The Escape menu's Load/Save/Delete/New Game rows now route to active workflows. Focused menu routing and save deletion pass; `cargo test --locked -p ffe-rs --test prefs` passes all 13 tests after correcting the stale intensity click and yellow-pixel assertion. A configured release-launcher run reached the landed game, but injected Escape presses after explicitly focusing the game window did not visibly pause or open preferences. This does not establish whether the application or automation input delivery is at fault; live menu verification remains incomplete. Broad `ui-5` stays open. Preserve the separate Display Options behavior in `prefs.rs`, `tests/prefs.rs`, and `docs/notes/display.md`.

## Partial progress — dashboard `flight-6`

**Gap:** the port had distance caps that switched the console acceleration step near planets, stations and starports. Original JJFFE F208 applies the selected step with no distance check, and F1013 changes takeoff mode by altitude without changing time acceleration. The caps are removed (owner, 2026-11-18); time returns to 1x only through the source's F217 events (docking completion, station/pad state, event messages, jump arrival, death).

**Acceptance:** the selected meter step is never limited by distance, on takeoff, in manual flight or under the autopilot; the F217 resets match the source; record the source comparison.

**Relevant files:** `crates/ffe-rs/src/main.rs`, `crates/ffe-rs/src/console.rs`, `docs/DEVIATIONS.md`, `docs/NAVIGATION.md`.

**Smallest verification:** `cargo test --locked -p ffe-rs --lib console`.

**Verified:** JJFFE source comparison recorded; player-selected acceleration is no longer forced to 1x or distance-capped anywhere (EXP-021 withdrawn, 2026-11-18). `cargo test --locked -p ffe-rs --lib console`, `cargo check --locked -p ffe-rs --bin ffe-rs`, `cargo clippy --locked -p ffe-rs --bin ffe-rs -- -D warnings`, `rustfmt --check` for `console.rs`, and `git diff --check` pass. `flight-6` remains open for its wider time-acceleration/RNG scope.

## Partial progress — dashboard `gameplay-39`

The two autopilot control tests, four cache-backed runtime tests and focused cancel test pass. Leave in progress: linked PAR-11 still lacks avoidance, moving-target pursuit, rotating-parent guidance and event-cadence replay.

## Partial progress — dashboard `gameplay-12`

The focused communication/messages frame test passes with the local content cache. Military offer constructors and remaining message behavior are still noted as gaps; leave in progress.

## Partial progress — dashboard `ui-7`

The 9 front-end tests pass and the intro-reference target compiles. With Skip Intro disabled, the configured release app reaches the animated intro splash/credits, but live key `1` did not visibly leave the intro. Original key `4` maps to Commander Load; direct `L` is a port-only shortcut and not an acceptance requirement. The key-4 handoff still needs verification. DOSBox-X ran `game/FIRSTENC.EXE` from a read-only mount and saved `.cache/ui7-original-20261116/dosbox-intro.png`; `--capture-intro` produced 13 port frames in `.cache/ui7-port-20261116` (including a model-176 undefined child-basis warning). The captures are from different intro moments, so a matched source-frame comparison remains outstanding. Leave in progress.

## Partial progress — dashboard `gameplay-34`

The three non-fixture player-event tests pass. The 96-case source comparison cannot run without `FFE_SERVICE_RECORDS`; the source audit still lists combat-score state and F952/F953/F950 routing gaps. Leave in progress.

## Partial progress — dashboard `gameplay-dust`

**Gap:** dust now spawns, ages and reaches the renderer, but `EXP-020` still says it is not drawn. Full source lifecycle/view-space culling parity has no focused render comparison.

**Acceptance:** correct the stale deviation text to reflect the current render path, and keep `gameplay-dust` open until a source-backed view-space/culling comparison passes.

**Relevant files:** `crates/ffe-rs/src/main.rs`, `crates/ffe-rs/src/screen.rs`, `crates/ffe-core/src/dust.rs`, `docs/DEVIATIONS.md`, `docs/notes/dust.md`.

**Smallest verification:** source/core dust unit tests are already recorded as passing; `git diff --check` for the documentation correction.

**Progress:** corrected the stale `EXP-020` statement: the renderer submits model `0x6d`, while source-equivalent view-space/culling behavior remains unverified. `git diff --check -- docs/DEVIATIONS.md` passes. `gameplay-dust` remains in progress pending the source-backed renderer comparison.

## Partial progress — dashboard `parity-01`

**Gap:** F938/F929 reserve consumption and repair cadence need full executed-source replay across idle, thrust, firing, and acceleration rates; the audit identifies whole-second event timing and wrapping arithmetic as the key parity gates.

**Acceptance:** compare event counts, reserve, warning time, restored thrust limits, shields and repair across the audit's 1x/10x/10000x cases; preserve source arithmetic and close only if the full trace evidence exists.

**Relevant files:** `crates/ffe-rs/src/main.rs`, `crates/ffe-rs/src/flight.rs`, `crates/ffe-rs/src/combat.rs`, `crates/ffe-rs/tests/damage.rs`, `docs/audits/gameplay-parity-2026-10-26.md`.

**Smallest verification:** `cargo test --locked -p ffe-rs --test damage`.

**Progress:** `cargo test --locked -p ffe-rs --test damage` now passes 18 tests after correcting the service-roll fixture under `gameplay-34`; no F938/F929 logic changed. PAR-01 still needs the source-backed multi-rate event trace, so remains open.

## Parked contract — dashboard `v0.48-8`

**Gap:** surface quads lack a source oracle at planet-scale distance.

**Acceptance:** produce a bounded executed-source comparison for the surface quad projection at the target distance, then close only if the relevant source output and Rust result match.

**Relevant files:** `crates/ffe-rs/src/model_viewer.rs`, `crates/ffe-rs/tests/planet_view.rs`, `tools/reference/verify-surface-near.py`, `docs/notes/planet.md`.

**Smallest verification:** inspect the existing near-surface oracle's coverage and run its narrowest case if its reference binary and vectors are available.

## Partial progress — dashboard `v0.48-8`

`tools/reference/verify-surface-near.py` requires four `--surface-near` records from an executed-reference binary plus the world cache. No reference executable or saved `surface-near.jsonl` exists in `.cache`; leave the item open.

## Historical UI-5 check (before configured-launcher capture)

Preference tests passed 13 cases and `--capture-prefs` produced a main-page frame with active Load/Save/Delete/New Game labels. The configured-launcher and scripted `--capture-start` results are recorded in the active contract above.

## Partial progress — dashboard `parity-07`

**Gap:** the Eagle's real muzzle origin is behind the front camera and model 156 expands into a screen-filling fan; the Saker currently shows one narrow ray. The owner requests the same single-beam presentation for all ships, with appearance still driven by the equipped laser.

**Acceptance:** every ship's front view shows one narrow beam, never a screen-wide fan or two fixed rays. Preserve model 156, the active mount's laser code/type appearance and the real simulation muzzle/hit origin; any camera-depth adjustment is render-only. Keep `parity-07` open for its broader Hope start/cockpit-image scope.

**Relevant files:** `crates/ffe-rs/src/screen.rs`, `crates/ffe-rs/tests/combat_view.rs`, `docs/notes/combat.md`.

**Smallest verification:** with the local cache, run `combat_view::f1776_model_156_draws_in_front_and_external_cameras` and `cargo test --locked -p ffe-rs --lib beam_view_tests`.

**Progress:** `draw_beam` moves only any behind-eye rendered origin to 0.33 flight units ahead of the camera; beam code/length and simulation muzzle remain unchanged. The ignored Eagle/Saker/external comparison passes, including an Eagle-vs-Saker pixel-footprint bound and laser-code variation checks. `beam_view_tests` (1) and the ignored 60 Hz firing/damage sequence (1) pass with `.cache/parity07/data.bin`; `rustfmt --check` and `git diff --check` pass. `--locked` cannot run because the existing lockfile omits 16 audio dependencies; unlocked resolution was used for checks and `Cargo.lock` was restored unchanged. The JJFFE source shows model 156 at the muzzle with the active laser code/length, but does not confirm a pixel-exact cockpit width; no original reference binary is available for image comparison. `parity-07` stays in progress.

## Partial progress — dashboard `gameplay-34`

The F934 regression fixture now uses one day remaining before the decrement and a fitted drive; its assertions accept the first deterministic equipment-or-drive failure and match the resulting state. `cargo test --locked -p ffe-rs --test damage` passes all 18 tests. No service logic changed. Keep `gameplay-34` in progress: the 96-case source trace and remaining F952/F953/F950 routes are still incomplete.

## Partial progress — dashboard `world-8`

**Gap:** system/galaxy map navigation still lacks the original details page (F842), picture mode (F844-F849), string templates, hover-pick, lit-sphere branch and drawn hyperspace transit scene.

**Acceptance:** finish the remaining source behaviors and compare the affected map state/frames against the executed original; keep the broad ID open until those gates pass.

**Relevant files:** `crates/ffe-rs/src/galaxy_map.rs`, `crates/ffe-rs/src/map_keys.rs`, `crates/ffe-rs/src/screen.rs`, `crates/ffe-rs/tests/galaxy_map_view.rs`, `docs/notes/galaxy-map.md`.

**Smallest verification:** inspect existing map frame tests and available reference/cache artifacts before choosing a focused change.

**Progress:** all five ignored `galaxy_map_view` tests pass with `.cache/parity07/data.bin`. They cover the map render, console-page states, key script, destination/jump frames and nearest-star selection. The notes still list the original details and picture modes, map hover pick and lit-sphere branch as unfinished; no source reference executable is available here. `world-8` stays in progress.

## Parked contract — dashboard `flight-6`

**Gap:** the manual takeoff acceleration correction is verified, but the broader time-acceleration/RNG item lacks parity traces across rate boundaries, frame rates, pauses and focus changes. PAR-12 defines the unresolved scheduling and event-ordering gate.

**Acceptance:** compare identical scripted inputs and RNG consumption at 20/50/60/144 fps, pauses, focus loss and acceleration boundaries; match source timer domains and event ordering. Keep the item open until the complete trace passes.

**Relevant files:** `crates/ffe-rs/src/main.rs`, `crates/ffe-rs/src/original_flight.rs`, `crates/ffe-core/src/player.rs`, `crates/ffe-rs/tests/flight_reference.rs`, `tools/reference/headless.c`, `tools/reference/verify-flight.py`, `docs/audits/gameplay-parity-2026-10-26.md`.

**Smallest verification:** run the focused `flight_reference` comparison with the executed-source vectors and world cache.

**Progress:** the F208/F1013 manual takeoff portion is source-compared and verified; the distance caps are removed entirely. The checkout has the original source (`jjffe-master`, JJFFE 2.8a7 assembly) and a built reference at `.cache/jjffe-reference/jjffe-smoke.exe`; the broader `flight_reference` gate was not run for this change. `flight-6` stays in progress. `gameplay-12` remains in progress for the separate military-offer/message gaps documented above.

## Readiness queue — not additional active work

| Existing ID | Readiness / unresolved gate |
|---|---|
| `gameplay-26` | Source starts 0/1/2/3/5 matched original fields. Review F2/hold behavior and affected old tests after correcting Ross 154 capacity. |
| `ui-7` | Intro oracle passed 76 cases. New source start choices and native load picker need UI/flow validation. |
| `flight-5` | Existing keyboard replay passed 730 frames; review remaining mouse/remap/module gates before closure. |
| `gameplay-34` | Servicing matches 96 original cases. F952 cargo gate and F953/F950 RNG/hit routing remain unfinished: NOT ready to close. |
| `gameplay-dust` | Needs source lifecycle/view-space culling audit; do not treat existing rendering tests as full source parity. |
| `gameplay-12` | Military offer constructors need target-ship and military-base state absent from the port; this expands the subsystem and requires source-state comparisons. |
| `release-5` | Clean-machine player-data acceptance is not defined by the bounded project evidence; keep not started until its actual import path is identified. |
| `release-4` | Audit scoping started; package outputs and acceptance criteria remain undefined. License and owner sign-off are separate open items. |

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

## Parked investigation — dashboard `gameplay-41` (bounded F669 event-order investigation)

**Gap:** F669 attacker-creation event 0x17 is currently raised after the script run; source notes identify this as a remaining ordering difference. Full item scope also includes mission-script import, journal behavior and scripted missions.

**Acceptance:** establish event 0x17's source statement timing and, if the runtime architecture supports an exact fix, raise it at the matching action point with a regression. Do not close `gameplay-41` unless the journal controls/timing and full scripted-mission acceptance are also verified.

**Relevant files:** `crates/ffe-rs/src/main.rs`, `crates/ffe-rs/src/scripts.rs`, `crates/ffe-rs/tests/mission_scripts.rs`, `docs/notes/mission-scripts.md`.

**Smallest verification:** `cargo test --locked -p ffe-rs --test mission_scripts -- --ignored` with absolute `FFE_WORLD_CACHE` set.

**Progress:** investigating whether the existing script hook can emit 0x17 synchronously during attacker creation. No code or dashboard status changed yet.

**Investigation result:** synchronous F669 event delivery would require materializing attacker objects during interpreter execution; the current host only queues abstract ship actions, and `apply` materializes them after the run. This is a runtime architecture change, not a small ordering patch. No code changed; keep `gameplay-41` in progress and retain the known journal control/timing gaps above.


## Latest completion — dashboard `world-9`

**Gap resolved:** the dashboard note predated source shade-table loading and F1535 class selection. The current renderer uses the local star vector on each required object path and the planet terminator follows the same system light.



**Relevant files:** `crates/ffe-rs/src/main.rs`, `crates/ffe-rs/src/screen.rs`, `crates/ffe-rs/src/model_viewer.rs`, `crates/ffe-rs/tests/planet_view.rs`, `crates/ffe-rs/tests/intro_view.rs`, `crates/ffe-core/src/model_lighting.rs`, `docs/notes/palette.md`.

**Smallest verification:** run `planet_view::local_star_light_changes_the_scene` and `planet_view::planet_surface_has_a_day_night_terminator` with the absolute local `FFE_WORLD_CACHE`; inspect the flight render routes for ship/station/lift use of `scene_light`.

**Delivered:** source-signed local-star lighting for ships, stations and lifts; F1535's six imported shade tables selected from the active star class; planet day/night shading.

**Evidence:** with `.cache/parity07/data.bin`, `planet_view::local_star_light_changes_the_scene` and `planet_view::planet_surface_has_a_day_night_terminator` pass; `ffe-core`'s `model_lighting` test passes. The extended `intro_view::returning_to_intro_drops_the_flight_star_tint` test verifies all six imported shade rows and Ross 154's flagged class `0x81`. Main routes the same F1481-signed light through ship, station and lift draw paths. Dashboard totals: 224 total, 152 done, **72 remaining**; `world-9` done.

**Changed files:** `dashboard/burndown.js`, `docs/BURNDOWN.md`, `docs/WORK_HANDOFF.md`, `crates/ffe-rs/tests/intro_view.rs`.

**Note:** classes 6 and 7 index past the six defined source shade tables; the port retains its documented default-shade fallback for those out-of-table values.
