# Accepted parity

Parity items that were closed on the dashboard as **close enough to the original
(about 90 to 95%)**, with the gaps that were knowingly left. This is the register of
acceptable parity: an item listed here is `done` in
[`dashboard/burndown.js`](../dashboard/burndown.js) by the owner's decision, not
because every line of its acceptance criterion in
[audits/gameplay-parity-work-items.json](audits/gameplay-parity-work-items.json)
was met. The residual gaps are recorded so a later bug can be traced back to a
known, accepted difference instead of being rediscovered.

Owner decision: 2026-11-18 (in chat: close what is about 95% close, note it, and
record the accepted parities somewhere; later the same day: finish `parity-01`, `-05`,
`-10` and `-11` the same way where testing shows them about 90% close). Same footing as the 2026-11-17
`ui-7` closure under the owner's authorization.

If an accepted gap turns out to matter in play, reopen the item (set it back to
`in-progress` on the dashboard, append a history point saying so, and move its
row out of this file).

## Closed with full evidence (no accepted gap)

`parity-02` reserve refuelling and cargo fuel bookkeeping, `parity-03` the
fitted-drive and failure state, `parity-06` set-speed controls, `parity-16`
system-wide traffic and police placement, `parity-19` the starting hold and
`parity-23` no silent prototype fallback. Their evidence is in the work-item
JSON and [PORT_LOG.md](PORT_LOG.md).

## Accepted at about 95%

| Item | Evidence it rests on | Accepted gap |
|---|---|---|
| `parity-04` takeoff handler (F1013) | `tests/takeoff_replay.rs` replays the executed original's launch frame by frame for Ross 154 (start type 1) and Hope (type 0): every mode change, set speed and hand-off frame is exact, velocity within 1 to 3 units, speed within 0.3% to frame 899. The no-key launch (`--flight hover`) hands off 0x24 to 0x26 on the same frames (315 Hope, 292 Ross). | The **engines-off request** (F1043, damping toward the combat velocity) is not the original's: with engines cut at frame 300 the original's velocity steps by about (+26, 0, +37) units in one frame and the port's barely moves. With the six thrust limits halved at frame 200 the port tracks within 1 unit for 47 frames, then drifts to 5 units (0.6%) by frame 248. |
| `parity-07` Hope start and cockpit view | The start frame's HUD text, message window and hull line match the executed original pixel for pixel; the pad seat matches `--geometry` exactly; the setting star now draws over the buildings from the one depth-sorted queue. The frame still differs by about 766 of 64,000 pixels (1.2%). | The star's top outline, a small red-roofed object near (52-65, 76-82) that is still unexplained, and the scanner's stalk, which comes from the nine-ship pirate group the port puts on a stand-in orbit (traffic placement, tracked by `parity-15`/`parity-17`). |
| `parity-09` orbital docking | The corridor test, speed limit and teleporting push are gone; F569/F570/F575 scan the station's collision stream. `tests/launch_replay.rs` reproduces every docking stage start (frames 7660, 8100, 8225, 8320, 8446, 8572, 8698, 8824, 8916, 8917) of the executed Lave Station dock. The player's autopilot approach (frames 1090 to 7660) matches within 0.2% every frame (`parity-11`'s first slice). | F595's station orbital velocity (the tick does not subtract it); other station models and a moving station have no comparison against the original, only Lave Station. |
| `parity-13` hyperjump | F909/F910 mis-jumps are traced in the executed original (`jjffe-smoke --misjump`) and ported on the shared random pair in the source's draw order; the hyperspace oracle re-runs and passes. | No whole-jump trace: F908's start needs the galaxy map's selection words, which the harness does not fill. Cancellation/F48 hooks, the mission-script outcomes 0/1 and the post-mis-jump map position remain unchecked. |
| `parity-14` fuel scooping | F1033 was read against the port: contact range, fitted bit, 30 km, speed 0x2328, the 0x2468a-tic interval, the used/capacity test, 0x9906 and the silent sample match; star heat and F1007's cooling are ported; F1014's drag and heating run in flight. | **No executed scoop trace.** The attempt destroyed the ship on its first frame because F569's star contact radius is not the +0x140 word. The star's contact is still the port's 30 km surface test, not F569's contact flag, so scoop distance and heat near a star are source-read, not original-measured. This is the weakest acceptance in the list. |
| `parity-01` event units, shields, hull, energy | F938/F929's arithmetic: 192 executed cases (reserve, shields, hull, wrapping, fuel, waste). The reserve from the new game through idle, the 0x24 climb and full thrust matches the original exactly to frame 484 and within 6 units to frame 899 (Hope: 12). Firing (`FFE_FIRE_FROM`, the real F931 every frame from 280, 21 shots over 728 frames): the refire word and the laser heat match on every frame and the reserve within 6 (Hope: 68, one thrust event, 0.4% of the spend). F152's whole-second events match on every frame of a 358-frame run through 1x, 10x, 100x, 1000x and 10000x and back (`tests/event_replay.rs`), the 43,200-event first dispatch and F208's pulled-back event included. | Shield recharge, hull repair, the energy warning's time and limit restoration are checked by the 192 oracle cases only: the Saker and Eagle starts fit no shield, so no live trace shows them. The accelerated steps were traced on the pad, not in flight (the scripted ship strikes the ground at 1000x). |
| `parity-05` launch frame and contact rules | Pad release, the pad seat and the rotating-frame climb replay the original frame by frame; the 3,119-frame climb and fall (`--flight takeoff 4000`, `port_long_flight_follows_the_original_to_the_descent`) keeps every mode and the set speed exact and the speed within 0.5% to frame 2800. The original's frame selector never leaves the planet's frame in that flight, and the port stays in it too. Return to a pad is replayed under `parity-11` (planet starport) and `parity-09` (orbital station). | The port's fall is steeper after frame 2000: its altitude is 20% lower by frame 2800 and it reaches the ground about 25 frames (0.5 s) before the original does, then bounces where the original explodes (frame 3120, mode 0x2c). No scripted flight made the original change frame, so that branch is untested but not contradicted. |
| `parity-10` indoor launch | The Lave Station launch (`--start-key=0x04 --flight launch`) replays frame for frame: modes 0x1a, 0x1c, 0x1e, 0x20, 0x22 and 0x36 on frames 170, 262, 388, 514, 640 and 735, the door opening on 861, the seat (0, 0, 0x10000) and the exit flight within 1.3% of the distance and 1.0% of the speed. | F595's orbital velocity (the ship never leaves the station's frame in 2,800 frames), the Thargoid models' reversed launch, other station models and a moving station: only start type 3 begins in a station, so there is no second station to compare. |
| `parity-11` autopilot | The player's approach to Lave Station follows the original within 0.22% of the 4.1 km leg on every frame, then the docking stages replay exactly. The whole descent to a planet's starport (`--start-key=0x02 --flight paddock`, `port_autopilot_flies_the_descent_to_a_pad`) replays over 14,934 frames: the port flies the 8 km descent to the pad within 5% in the first 150 frames after the engage and within 1.2% (50 to 65 m) after frame 1570. | F610/F802 obstacle avoidance (no obstacle in either run), pursuit of a moving ship, the autopilot at 10x to 10000x and the Hope descent (the original's scripted Hope launch crashes at frame 648). The port's detour and waypoint additions are unchanged. |
| `parity-12` event clocks and time acceleration | F152's whole-second events replay against the executed original on every frame of a 358-frame run through 1x, 10x, 100x, 1000x, 10000x and back (`tests/event_replay.rs`, on a corrected harness: F208's pull-back now moves the real marker), the 43,200-event first dispatch included. The same takeoff at 20 and 21 ms frames differs in the original as it does in the port (`port_takeoff_replays_an_uneven_frame_rate`). A 1x to 10000x schedule at 20, 50, 60 and 144 fps gives the same milliseconds and Timer50Hz callbacks and events within 2 of the ideal; a pause calls no timer and a stalled window is one 0.1 s frame (`ffe-core` `frame_timer_tests`). | The prototype accumulator, the event-6 controllers on Timer50Hz and the collision work at high acceleration have no comparison against the original; the acceleration was traced on the pad, not in flight. |
| `parity-17` shared random pair and call order | The pair is where the executed original leaves it through boot (F622 and F688 on two starts, `bbs_reference.rs`), through the quiet pad and climb (no draw in 244 frames, none in the climb), and through a whole new day: with the original's own draw log (`jjffe-smoke --flight idle`, `FFE_RNG_TRACE`, now F922 and F1530) the day's 125 draws (three news chances with F623's draws, the place's 99 market steps, the replaced board entry and its F621 handler, the ship that leaves, the three F634 chances) are the port's, and the pair ends at `69a7fb5e d78a9136` as the original's does (`tests/rng_ledger.rs`). The new-game day now changes at midnight, twelve hours in (it changed at twenty-four), so that day's draws come when the original makes them. | The 99 market steps are consumed, not read (EXP-006): the market's prices and stock still come from each station's own stream, so they differ from the original's although the pair is right; a visited station's board and hull list change from its own stream (the shared-pair copy of every board is kept and exact); the news rounds use rank 0 for both ranks; the F1476 jitter draws are isolated by decision (SIM-010); the damage, servicing and combat rolls in flight and the hyperjump arrival draws have no draw-by-draw trace against the original; only one start (Ross 154) was traced across a day boundary; the harness's `--flight` boot leaves the pair at `a3ce90ba 5ad9a9b2`, not the port's after F688, so the day was compared from the original's own pair. |
| `parity-18` complete live state in a save | The native save carries the live NPCs and stations' bay records, missiles, chaff, effects, cargo pods, the shared pair, the calendar and queued day events, the shipyard and bulletin state, deadlines, the autopilot's history and clearance retry, the frame selection and mid-jump transit (persist-1 to persist-25, twelve cache-backed scenarios with migration and malformed-state rejection). New: the integer flight words (rates, sub-unit fractions, caps, thrust words, drift, scanner) travel with the save (`Flight::resume`), so eight resumes in 400 frames of steering give the identical `Motion` every frame while a resume without them differs on the next frame; F633's working copy of every board is saved, and a resumed navigator and an uninterrupted one draw five days to the same pair, boards, jobs and remembered destinations (`navigation_continuation_tests.rs`). | A save while the pad world (the takeoff climb in the planet's frame) or the autopilot's guidance is live rebuilds the integer state; the jump transition and the landed surface were not compared frame for frame against an uninterrupted run; the steering-key hold counters are not saved (a save is made from a menu); the sub-millisecond clock carry is not saved. |

## Linked flight and gameplay items closed 2026-11-18

Owner request (in chat, 2026-11-18): close the ten lowest-effort open items. Nine of them are the flight and autopilot
items whose accepted parity twins are in the table above; they close on the same evidence and the same gaps. Every replay
was re-run the same day against a freshly executed original (`jjffe-smoke.exe`, traces written to a scratch folder), not
taken from older notes: takeoff (clear at frame 430 in the original, 429 in the port; largest speed difference 0.18%), the
same takeoff at an uneven 20/21 ms frame (437 against 436), the no-key hover (hand-off frame 292 on both), keyboard set speed,
the reserve and 21 shots of firing (largest reserve difference 6), the 2,631-frame climb (worst speed 0.48%), the Lave launch
stages, exit flight, autopilot approach and every docking stage, F152's events through 1x to 10000x, and the 14,934-frame
descent to a pad (worst 4.93% of the 8 km descent at frame 914, 1.20% after the first 800 frames). `ffe-core` (205 tests),
`tests/autopilot.rs` (6) and the dust tests also pass.

| Item | Rests on | Accepted gap |
|---|---|---|
| `flight-4` fixed step decoupled from the frame | `frame_timer_tests` (20/50/60/144 fps give the same milliseconds and Timer50Hz calls), the uneven-frame takeoff replay | The original steps physics by the frame's tics, as the port does; the prototype accumulator, event-6 controllers (20 ms of game time rather than Timer50Hz calls) and collision work at high acceleration have no comparison. |
| `flight-6` time acceleration and RNG | `event_replay.rs` (every frame through 1x to 10000x and back, the 43,200-event first dispatch included); the takeoff replay asserts no draw from the shared pair | Traced on the pad, not in flight; the RNG order of the whole event loop belongs to `parity-17`. |
| `flight-9` rotating station frame | exit flight in the station's frame (frames 861 to 1089) and the docking stages | F595's station orbital velocity; Lave Station is the only station compared. |
| `flight-10` docking rules, station pose | every docking stage start of the Lave dock; the approach within 0.2% | As `parity-09`. |
| `flight-11` launch sequence | the Lave launch stages 0x1a to 0x36 and the pad launch hand-offs | As `parity-10` and `parity-04`. |
| `flight-12` scripted scenarios traced against the reference | the eleven replays above, from takeoff to the 14,934-frame descent | Combat, hyperjump transit and scooping have no executed trace (`gameplay-8`, `parity-13`, `parity-14`). |
| `flight-13` open-air starport takeoff | takeoff and long-climb replays; pad descent and landing | The ship returns to the system frame at the clear while the original keeps it a child of the planet (no visible effect on speed); the engines-off request. |
| `flight-14` original autopilot | the pad descent and the Lave approach | As `parity-11`: F610/F802 avoidance, pursuit of a moving ship, 10x to 10000x. |
| `gameplay-39` autopilot at run time | the same two replays, `tests/autopilot.rs` (4 local-cache and 2 synthetic cases) and the cancel test | As `parity-11`. |
| `gameplay-dust` space dust | The executed original's 45 dust clouds at 9,994 km/s drawn through the port: 153 port pixels against 151, all 151 covered, 42 single pixels and 11 "+" shapes in both. The speed class (F1475, `DATA_008865`) and the F1751 detail words (`DATA_008812/3`) are ported; 14 core and 7 cache-backed dust tests pass (`docs/notes/dust.md`). | The port draws a Milky Way band where the original probe frame had none; the port culls and ages dust each frame while the original does so on F152's day tick (a cloud can live up to a second longer); two port pixels under the crosshair. |

Not closed with these: `flight-1`, `flight-5` (the original's remapping is not ported), `flight-8` and every other open
item.

## Second batch closed 2026-11-18

Owner request (in chat, 2026-11-18): close ten more low-effort items. Re-run the same day: the oracles `hyperspace`, `market`,
`sectors`, `system-names`, `galaxy-map`, `surface`, `pads`, `world`, `world-tick`, `collide`, `combat`, `physics`, `thrust`,
`thrusters`, `ship-types`, `takeoff` and `flight` of `tools/reference/verify-all.py` (each against the executed original) all
pass, and so do the `crash`, `damage` (18), `death`, `death_view`, `fitting`, `landing_load`, `market_view`, `trade_loop`,
`gameover_view`, `station_dock` and `timed_events` tests and `ffe-core`'s market, landing, legal, galaxy and status tests.

| Item | Rests on | Accepted gap |
|---|---|---|
| `world-4` galaxy generation, RNG stream question | `sectors`, `system-names`, `galaxy-map` and `world` oracles; the 2026-10-23 owner decision (drawing keeps its own jitter stream, `KNOWN_ISSUES`) | Outcomes that depend on how many numbers the original's drawing consumed (combat rolls, agent behaviour) will not match roll for roll; that is the recorded decision, tracked by `parity-17`. |
| `world-5` planet surfaces and landing approach | `surface`, `world`, `pads` oracles; the pad-descent and climb replays; `landing_load` | The frame switch in a long flight is untested (no scripted flight left the planet's frame); the fall after frame 2000 is steeper than the original's (`parity-05`). |
| `world-7` hyperspace and arrival | `hyperspace` oracle (8,192 cases of F903) and the F909/F910 mis-jump trace | No whole-jump trace (F908 needs the map's selection words); cancellation hooks and the post-mis-jump map position are unchecked (`parity-13`). |
| `gameplay-1` market and pricing | `market` oracle (8,192 executed cases), `market_view`, `trade_loop` | F542's pay-fine screen is a one-line message; the smuggling scan runs at launch, not at the original's special-slot write; mission cargo and F641's special-offer commodity are not traded. |
| `gameplay-3` fuel, damage and repair | `parity-01`/`-02`/`-03`/`-14` evidence, `damage`, `crash`, `fitting`, the reserve and firing replays | Shield recharge and hull repair are checked by the oracle's cases, not a live trace (the Saker and Eagle fit no shield). |
| `gameplay-34` subsystem damage, F938/F929 repair rows, crash damage | the same, plus `collide` and `crash` | As `gameplay-3`; crash damage is compared at F953's amount, not over a whole crash trace. |
| `gameplay-13` player death and the escape capsule | `death` (9), `death_view`, `gameover_view`; the executed death probe (`docs/notes/death.md`) | The commander name is a constant (`Jameson`); the obituary lines and palette are not pixel-exact; the pixel comparison waits on `parity-08`/`-17`. |
| `gameplay-25` F1104 hold, F1098 escape craft, capsule launch, GAME OVER strings | as `gameplay-13` | As `gameplay-13`; the mode word `+0xe8 = 0x24` is deliberately not set. |
| `release-5` clean-machine import | `tools/clean-machine-check.ps1`: a new folder with only `ffe-rs.exe` and a copy of the game imports, starts twice, leaves the game byte-identical and adds nothing but saves, settings and logs; an empty game folder is refused with exit 2 and a message naming FIRSTENC.EXE | Floppy 1.02 only (the CD 1.01 executable is not in this checkout); the game imports in memory on every start, so there is no cache file to reuse; the script cannot prove a machine without developer tools, so run it from the packaged zip on the target machine. |
| `release-4` packaging and distribution audit | `tools/package.ps1` (executables, documents, `SHA256SUMS.txt`, third-party notices and licence texts; refuses player data and generated files), `tools/third-party.py` and `docs/THIRD_PARTY.md` (223 crates; checked in CI), `ci/check_forbidden.py` | **Owner decisions remain:** the three LGPL-2.1 `oxisynth` crates are linked statically, so a binary release needs a relinking arrangement; the 32 MB GeneralUser GS sound bank is compiled into the executable and its author cannot vouch for every sample's origin; the Rust code's own licence (`release-6`) is unchosen. |

## Third batch closed 2026-11-19

Owner request (in chat, 2026-11-19). Re-run the same day: the `traffic`, `station-queue`, `station`, `station-models`, `lift`, `npc`
and `pads` oracles against the executed original pass; F934's servicing matches the executed original on all 96 dispatches
(`player_events`, vectors from `--legal-probe` with `FFE_SERVICE_RECORD=1`); the `fitting` (6), `ship_change` (3), `station_dock` (4),
`station_view` (12) and `station_traffic` tests and the outfitting and trade unit tests pass.

| Item | Rests on | Accepted gap |
|---|---|---|
| `gameplay-2` equipment and ship purchase | the equipment, ship-change and fitting tests; the `market` oracle's tables | No live trace of a hull purchase or outfitting session against the original; the mining machines' deployment (F242-F252) is ported but not compared; laser mounting choice uses keys 1 to 4 (`DEVIATIONS.md`). |
| `gameplay-11` shuttles and station traffic | the `traffic`, `station-queue`, `station-models`, `lift` and `npc` oracles; `station_traffic`, `station_dock` | Station traffic uses the pad point, not the original's approach; F745/F746's orientation matching is approximated; per-state NPC frame matching belongs to `parity-15`. |
| `flight-8` player events and state machine | F934 against the executed original (96 dispatches), `player_events`, the takeoff, launch, dock and hover mode replays | The engines-off request (F1043); modes the scripted flights never reach (combat, hyperspace transit). |
| `world-6` starport services and events | `station_view`, `station_dock`, the `pads` and `station` oracles | F524-F532's negotiation pages (permits, advance) are not drawn (the offer is accepted directly); the BBS entry's service flags; the pay-fine screen. |

## Accepted at about 80%

Closed on the owner's request "finish 15" (2026-11-19), on the same footing as the items above but at a lower match: the part that
could be traced against the executed original is exact, and the rest has no original trace to compare with.

| Item | Evidence it rests on | Accepted gap |
|---|---|---|
| `parity-15` NPC speed caps and flight states | NPC ships with statistics fly by the original's object physics (F597, F601, F585, F1512). The pirate leader's F750 approach replays 2,268 frames (orientation, thrust, caps and rates exact, position and velocity within one unit; `npc_pirate_replay`, local). F238/F239/F240's police response replays frame for frame: Hope 7,798 of 7,798 (11 launches), an orbital station 4,798 of 4,798 (10 launches), including the F1530/F922 draws (`police_replay`, local). F748 acts on the ship's own step (143 frames after the launch). No entry spawns a police patrol; a script's F236 records the offended starport. | F702's system-wide placement (pirates stay on the stand-in orbit); F593's waypoint offset (about 1 km); attack states 1/2 and the missile rows 0x13-0x1b are untraced; F659's launch positions; F233's event 1; traders, escorts and heavy freighters were never traced because the original gives them no dynamics without the player in their frame. The port's speed caps and acceleration ramp remain only for ships without statistics. |

## Not accepted yet

`parity-08`, `parity-20`, `parity-21`, `parity-22` and
`parity-24` stay open. Their blockers are in the work-item JSON and
[STATUS.md](STATUS.md).
