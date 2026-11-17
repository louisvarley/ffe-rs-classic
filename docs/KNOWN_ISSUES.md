# Known issues

Policy (owner decision, 2026-10-20): **JJFFE 2.8a7 is the gameplay baseline;
verified bugs may be fixed.** Characterize the defect, add a regression test,
and record the intentional deviation in [DEVIATIONS.md](DEVIATIONS.md).
Default fixes are permitted; not every fix requires an opt-in toggle.
Enhanced graphics options remain presentation-only and never fix gameplay.

The previously studied FFED3D tree contains many `FIX:`/`ANISO` changes.
These are **not** automatically part of the selected separate JJFFE baseline.
The table below is historical comparison evidence, not a list of approved
changes. Reconfirm any suspected defect against JJFFE before treating it as a
port task. See [CLAUDE.MD](../CLAUDE.MD).

---

## 1. Historical FFED3D behaviour vs DOS 1.02 (not the selected baseline)

These are explicit changes in `ffe.asm`/the C++ modules. They are **not** bugs in the port; they are differences from the original DOS release. `docs/archive/PHASE0_INVENTORY.md` §9 has the full context.

| Area | Reference change | Source marker | Notes |
|---|---|---|---|
| Lasers | Laser invulnerability changes | `FIX: Laser invulnerability` (15387, 49257, 82125) | Affects damage windows, including docking. |
| Equipment price | “incorrect tech prices” | `FIX` (29251) | Shop price formula. |
| Collision | Collision radius | `FIX` (38375) | Affects collisions/landing. |
| Missiles | Missile retardation | `FIX` (38534) | Missile physics. |
| FOR | Linear FOR switch velocity | `FIX` (41000) | Frame-of-reference transitions. |
| Autopilot | Acceleration fudge | `FIX` (41172) | Autopilot behaviour. |
| Smoke | Mispositioned smoke | `FIX` (48827) | Presentation timing/position. |
| Combat | Combat velocity fudge | `FIX` (49299, 51998) | Velocity handling. |
| Pulse lasers | Pulse laser non-limitation | `FIX` (49683) | Fire rate/capacity. |
| Hyperspace AI | AI time travel during HS | `FIX` (50131) | AI timing. |
| Orbits | Orbital drift | `FIX` (62299) | Long-term orbital positions. |
| Binaries | Binary light / over/under adjustment | `FIX` (65628, 66497, 66516) | Star/planet lighting and orbits. |
| Starports | Starports below ground | `FIX` (66283) | Station placement. |
| Asteroids | Asteroids run out | `FIX` (69361) | Asteroid spawn/lifetime. |
| Hyperspace | Player hyperspace time-travel | `FIX` (69426) | Time accounting. |
| Calendar | Groundhog day | `FIX` (70432) | Date rollback. |
| Missiles | Ray-collision disabled | `ANISO BUGFIX` (72632) | Missile collisions. |
| Cargo | Cargo space not lost | `ANISO bugfix` (73056) | Cargo capacity on destruction. |
| Cabin | Cabin temp related to game time | `ANISO FIX` (77067) | Atmospheric heating. |
| Turrets | Turrets cannot turn when paused | `ANISO bugfix` (78838) | Pause behaviour. |
| Turning | “limit turning speed?” / AI turning | `ANISO` (40065, 40128, 40143) | Turning-rate model. |
| Pirates | No instant snap to player | `ANISO` (41141) | AI approach. |
| Missiles | Max thrust 120g instead of 60 | comment at `F601` | Missile performance. |
| Start | “Frontier 3500” extended universe | `FUNC_000829`, `DATA_ExtendedUniverse` | Alternate start; disables hand-coded missions/journal and uses C++ overrides. |

**Action:** preserve the selected JJFFE behavior except tested, documented
fixes. Do not silently import these FFED3D modifications.

---

## 2. Confirmed/suspected defects in the reference or original

These are logged as-is. They are not fixed by default.

1. **`FUNC_000930` damage path** — `ffe.asm:71792` comment: “calculate damage! BUG HERE”. The comment does not describe the defect; Phase 1/3 must characterise it and decide whether a trace-visible bug exists.
2. **`F381`** — `ffe.asm:82126` comment: “Unfixed - actual bug in F381”. Related to laser/docking invulnerability; needs characterisation.
3. **3D system map crash** — `ffe.asm:96427` comments out a test that “crashed in 3d system map”. A known crash path in the original UI; reproduce/record before fixing.
4. **Save/load edge cases** — the compressed/scrambled save is a defined subset; corrupt/truncated files may not fail cleanly in the reference. The port must fail cleanly (error, no panic) while preserving valid-file compatibility. This is a robustness fix, not a simulation change.
5. **Journal parsing** — the `MISSION.DAT` bit-permutation/offset table is only partially documented in the notes; the exact entry count and trailing bytes are not confirmed. Treat as a Phase 2 verification item, not a known game bug.
6. **Starport/collision edge cases** — the reference fixes “starports below ground” and collision radius; other placement/collision edge cases may remain. Phase 1 traces should include near-surface/landing cases.
7. **Time rollback/date** — the reference fixes “groundhog day”; other calendar/hyperspace time-accounting edge cases (e.g. time accel + save/load) should be traced.
8. **AI freeze/pause** — turrets cannot turn when paused (reference fix); other pause interactions (autopilot, missiles, station traffic) are untested.
9. **Input focus** — the reference uses DirectInput; losing focus/reacquiring devices can drop keys/buttons. The port must handle focus robustly; default bindings must still match.
10. **Audio device loss** — DirectSound buffers can be lost; the reference restores them. The port's audio backend must handle device changes without changing trigger timing.
11. **Large time steps** — the physics code caps accelerations and uses shift-based time scaling; extreme time acceleration can produce different paths. This is part of the reference; trace it rather than “fix” it.
12. **Window/aspect** — FFED3D's 16:10 `aspectfix`, 640×400 software buffer, TTF font and linear filtering are deviations from DOS, not bugs. Classic will not inherit them.

---

## 3. Phase 0 study-build limitations (not product bugs)

- The study binary uses VS 2026, NASM 3.02, a D3DX9 NuGet package, Wine `d3d9.h`, a DirectInput shim, `#pragma pack(1)`, and a splash bypass. It is not a faithful 2010 build.
- Scripted input is not working yet; the splash was bypassed for the smoke test. Full gameplay/saves/audio were not exercised.
- The reference build links the debug CRT in Release and disables optimisation; performance numbers from it are not representative.
- The source has no licence; the binary is not distributed.

---

## 4. Port-phase gaps (to be closed)

- An experimental native window now runs launch/fly/dock/trade/save/reload.
  It is **not faithful First Encounters**: synthetic wireframe station, stars,
  floating-point prototype flight, fixed one-item market and safety resets.
  V0.2 uses original cabin0, console artwork and main font in a 320x200 logical
  view with 4:3 correction. Functional original HUD/buttons/scanner, dynamic
  palette, full FF-code text and world/models are still absent. The green scanner
  contact is prototype math, not original scanner behavior. Other UI characters
  outside printable ASCII display as `?`; full diagnostics go to stderr.
  V1 caches can be inspected but must be rebuilt to v2 for the new font.
  V0.4 extends F4 with actual declared root triangle/quad outlines and single-face
  inspection (cache v4); v3 keeps its points-only lab. These catalogue all physical
  face records, including alternate branches, not a full program execution.
  V0.5 adds exact normal visibility/lighting and a diagnostic normal-filtered
  outline view (cache v5), not branch execution or shaded pixels.
  Original integer projection (F1479) and live vertex slots are verified (v0.10);
  v0.11 composes them with the depth commands/groups in
  `model_execute::execute_live`. No complete station command execution or
  submodels yet.
  V0.7 executes a verified bounded command subset on composed synthetic programs.
  V0.8 verifies the standalone original depth-key calculation; v0.9 adds separately
  verified depth-command resolution and flat group ordering. Actual station streams
  now advance past opcode 6 and stop at the next unsupported command (point/text/
  submodel/other transform). Nested child-model queues and real primitive storage
  remain absent.
  The lab still catalogues faces, not executor output. Arithmetic/bit-branch/material
  helpers are verified;
  integer trig and Q31 basis rotation are now verified standalone helpers (v0.6),
  with packed angle operands and pure local-basis rotation commands also verified.
  Base/object-relative and vertex-defined transforms remain unimplemented.
  Normal/LOD visibility predicates and branch outcomes are verified independently,
  composed in the bounded executor, not a complete station command interpreter.
  Test values are not live animation
  state. Gameplay still uses the synthetic cube. Unsupported vertex operations
  fail explicitly; the interpreter is not yet suitable for arbitrary game models.
  See EXP-001 through EXP-004 and the actual short tests in [README.md](../README.md).
  No audio, combat, planets, hyperspace, time acceleration, old saves, remapping,
  persisted settings or image packs. F5 creates a new numbered snapshot; no
  autosave or quit prompt. Failed writes can leave an invalid newest snapshot;
  F9 reports errors rather than silently loading an older one. Explicit cache
  creation is still required before launch. No faithful gameplay goldens yet.
- Rust inspection, partial content cache and first RNG/clock/counter primitives
  exist; no complete faithful simulation or renderer yet. Inspector validates exact journal/cabin hashes
  but only presence/basic shape for audio and counts for optional media.
- Local JJFFE startup smoke and selected new-game state traces exist, with
  differential RNG/timer vectors. The bounded 400-frame scenario validates 238
  game-clock transitions, not flight, docking, trading or save/load. Full state/
  pointer normalization, faithful pacing, input scheduling and clock wrap remain absent.
- Partial `data.bin` builder/reader imports 345 images, two raw texture blocks,
  main font (v2), two station vertex programs (v3), two root command streams (v4),
  two station normal blocks (v5), sine quarter-wave table (v6), low palette,
  cabins and PCM effects. Submodels, gameplay tables, additional fonts, journal,
  music/CD conversion, complete cache and automatic startup conversion remain absent.
- Repeatable bounded startup framebuffer hashes exist, not gameplay goldens.
- Floppy 1.02/CD 1.01 embedded table layouts match JJFFE bytes in both real inputs.
  Upper palette colours remain runtime state; no colour-guessed embedded image
  export or automatic upscale eligibility. The cabin-only preview export is
  unchanged. This is not full original-game asset/gameplay support.
  Patch-output fingerprints remain unverified.
  Direct BIN/CUE ingestion is deferred.
- No mod system yet (Phase 8).
- Windows/Linux Rust CI jobs configured for formatting, strict clippy, synthetic
  tests and release tool builds; not remotely executed during local development.
  No game packaging yet. Repository-hygiene checks exist.

---

## 5. How to add an issue

Use a short ID, affected subsystem, exact source reference, baseline/build,
reproduction steps, expected vs observed, and planned correction. Add a failing
test first and record any intentional behavior change; default fixes are allowed.

## F1476 state shared between vertex jitter and other code (decided 2026-10-23)

The original's vertex opcodes 7/8 draw from F1476 (`DATA_008818`/`DATA_008820`),
whose state other game code also reads (about 35 routines, among them the
physics, agent and system-generation code). Rendering therefore affects
non-rendering behaviour in the original.

**Owner decision (2026-10-23): isolate drawing.** The port keeps the jitter
state in the vertex cache, owned by the caller, so presentation never disturbs
the simulation. The simulation keeps its own stream with the original's
generator. Consequence: replays and saves behave the same whatever is on
screen, but outcomes that depend on how many numbers drawing consumed (combat
rolls, agent behaviour) will not match the original roll for roll. Recorded in
`docs/DEVIATIONS.md`.


## Station velocity is not ported (frames)

`ffe_core::frames` handles rotating frames (F1677, F572, F573) but not a
station's own velocity (F595 through F1533/F866), which returns `None`.
`ffe_core::collision` now covers every path of F569/F570: the distance and frame
switching, complex shapes (`+0x14c` negative; F576, F579-F583), terrain
(F581/F1876), F575's soft landing and crash/bounce tail (`DATA_008699`,
`DATA_008700`; F953's damage amount is returned) and F578's response to touching
another body. A touch needs the models' records and statistics (`Shapes::model`,
F1538); without them the scan returns `None`. One path still returns `None`: a
complex shape whose terrain stream ends between the planet's inner and outer
radii (`DATA_008707` 0xff) falls into F575's sphere placement, which reads
F577's squares from a local the F576 path never set (the original reads
uninitialised stack). Note that F577's far branch (distance beyond 2^30 units)
tests the sum of the bodies' outer radii (`+0x138`), not the collision radii
(`+0x140`) that its near branch uses (a JJFFE "FIX" applied to one branch only),
so a planet's atmosphere counts as contact at long range; the port keeps this.
Near an orbital station the playable now runs this code too: inside the
station's hull radius the ship is put in the station's spinning frame for the
tick (`OriginalFlight::step_station`) and F569 scans the station's collision
stream, so a soft touch in the station region (`0x20 | 0x40`) is F1021/F1024's
docking or no-clearance case and a hull strike is F575's crash and bounce. The
port's old corridor test and "pushed clear" teleport are gone. The scan uses the
station's spun rows (F866) and the ship co-rotates with the station frame
(F572). F595's station velocity itself is not ported: the navigator carries
the ship with the station's orbital motion instead (its frame following), and
the launch starts at F1088's seat but waits for the door in the lift stages
rather than parked at the seat (PAR-10).

## Eagle laser beam draws as a red fan (found 2026-11-17; fixed the same day, one-pixel ray left)

Firing any laser in the Eagle (ship 23: the Ross 154 and Sol/Mars starts; the
Mars start's `0xa0` beam laser shows it most, the pulse does the same) draws a
red fan at the crosshair instead of a thin ray. The Saker (ship 31) draws a thin
ray correctly.

Findings so far:

- The Eagle's statistics put the front gun behind the eye: eye `[1500, 30]`,
  front gun `[1300, 0]` (Saker: eye `[1800, 400]`, gun `[2200, 0]`). So model
  0x9c (F1776's beam) starts about 0.165 flight units behind the camera. The
  camera and `combat::gun_origin` use the same scale, so this is the data, not
  a unit mismatch.
- `Screen::draw_beam` clamps the render origin to 0.33 in front of the eye
  (`MIN_RENDER_DEPTH`). With the clamp the fan is small (the screenshot); without
  it the beam fills the lower half of the view. Neither is the original's thin
  ray.
- Polygon near-plane clipping (F1564) is ported and checked
  (`docs/notes/polygons.md`), so look first at how model 0x9c's ray polygons are
  built (the flare, the tube, global 4 = laser type, global 37 = length) and at
  what the original does with a muzzle behind the eye.

To reproduce: a local test that builds `Screen` with `lasers = [0xa0, 0, 0, 0]`,
sets `screen.beam = combat::beam(..)` for ship 23 and 31, draws a frame and saves
it (as `tests/laser_view.rs` does with `FFE_SHOT_DIR`). Not fixed; not verified
against the executed original.

Further findings (same day):

- Model 156's cull radius is 0x800000, so F1680 cannot cull the beam for a
  muzzle 0.165 units behind the eye. The ~64-unit muzzle cone lies wholly behind
  the near plane (z <= 0x40) and drops out; what remains is the forward ray.
- The ray is a strip parallel to the view axis. The Saker's gun is 3200 model
  units below its eye, so the strip reads as a thin line converging on the
  crosshair. The Eagle's is only about 240 units below (up word 0 vs eye 30), so
  the same fixed-width strip, clipped at the near plane, fans out to the screen
  edges. The geometry is therefore what the ported pipeline gives for these
  words; the render is not obviously a clipping bug.
- So the open question is whether the original shows this for the Eagle. Either
  the original differs in the gun/eye words it uses for the Eagle (check the
  ShipStats offsets `eye`/`front_gun` and F1776's use of them), or it limits the
  ray near the camera. Settle it with the executed original (the reference
  harness in `tools/reference`), not by tuning `MIN_RENDER_DEPTH`.

F1776 read against `ffe-rs-classic/jjffe-master/ffe.asm:111336` (same day): the
gun position is `(row >> 16) * word >> 12` along the forward row (front gun
words `+0x20`) plus the up row (`+0x22`), added to the object's 64-bit position
before F1680 draws model 0x9c, and `ShipStats` reads the same words
(`+0x18/+0x1a` eye, `+0x20/+0x22` front gun). So `combat::gun_origin` matches
the source. The remaining difference must be inside the draw of model 0x9c from
a muzzle behind the eye: next check whether the ray's faces are culled in the
original (F1480/F1481 observer position; `docs/notes/polygons.md` says the
port's observer comes from the diagnostic camera) and compare against a
reference run. No code was changed.

Render scale is the likely cause (same day, from the source and renders):

- F1776 does `F1533(local, object, 0)`, a 0x152-byte `memcpy` of the firing
  object (the player), changes only the model to 0x9c and calls F1680. F1680
  and F1683 shift by the object's `+0x7e`, so the beam is drawn at the *copied*
  `+0x7e`, not necessarily at model 156's own scale (17). F1536 does the same
  for the hit flash but sets `+0x7e` from the hit ship's model record first; the
  port copies that (`render_scene_with_scale`, ship scale) but draws the beam at
  model 156's own scale 17.
- Experiment (not kept): drawing the beam at the Eagle's/Saker's scale 8 (512x
  smaller) removes the fan. With the 0.33 clamp the Eagle shows a muzzle flash
  and a thin ray; with no clamp its ray is a thin line from the bottom toward
  the target when firing at a ship (`tests/laser_view.rs`), nothing on an empty
  sky; the Saker's beam becomes almost invisible (its gun is 3200 units below
  the eye, off the bottom of the view).
- Open: the player object's `+0x7e` at the moment of the shot (the assembly sets
  it in several places, e.g. 0 at creation `ffe.asm:71572`, a distance shift at
  `:70786`). Read it from the executed original, then draw the beam at that
  scale and drop `MIN_RENDER_DEPTH` if the result matches. No code was changed.

**Fixed (same day).** The executed original (`ffe-rs-classic/.cache/jjffe-reference/
jjffe-smoke.exe --game-frame <n> 0x02` with `FFE_TAKEOFF=1 FFE_FIRE_AT=600`, the
Eagle in flight) draws no beam pixel in the 3D view after a shot: the only pixels
that differ from the unfired run are the message line (rows 184-197). So the fan
was wrong, and the render scale above was the cause. `Screen::draw_beam` now draws
model 156 at the firing ship's scale and the depth clamp is gone (the Eagle and the
Saker are both scale 8). `tests/eagle_beam.rs` pins "no fan".

Saker checked too: `tools/reference/headless.c` now takes `FFE_START_KEY`
(`0x39` = the default Saker Mk III at Hope, model 31) in `--game-frame` mode, and
the original draws no beam pixel in the Saker's front view either (0 differing view
pixels against the unfired run, frames 601-603). So the Saker's nearly invisible
beam in the port is right, and the streak it showed at scale 17 was wrong.

Still different: for the Eagle the port draws a one-pixel ray (about 47 pixels,
from (260,120) towards (306,157)) and for the Saker a small mark at the bottom
right, where the original draws nothing. After a shot the original's `+0x151` byte
stays `04`, which F1776 would clear if it ran its front-gun branch, so check
whether F1776 returns early in the front view (and what it tests) before hiding
those pixels.

## Hit flash timing and the model-0xa1 glow (found 2026-11-17, parity-01 audit)

F938 (`ffe.asm:70122`) is ported in full for the player: shield recharge, hull
auto-repair, laser heat cooling, F929's thrust energy and the `+0x10e` hull line
countdown all match the source. Two ship-hit visuals in the draw loop F1536
(`ffe.asm:91262-91340`) are not the original's yet:

- **Model 0x9d flash length.** The original draws it once: F1536 clears
  `+0x151 & 1` as it draws, so the flash lasts one drawn frame. The port holds it
  for `FLASH_SECONDS` = 0.25 s (`ffe_core::npc`). Not changed: one frame at the
  port's 60 Hz presentation would be much shorter than the original's, and the
  original's frame time has not been measured.
- **Model 0xa1 glow.** While `+0x14f != 0` (and `+0x151 != 0`) F1536 first draws
  a copy of the ship as model `0xa1` with random orientation (F1533, `MatBuildYZT`
  with two F1530 draws). `+0x14f` is set to 2 (F935, `ffe.asm:69995/70009`) or 7
  (`ffe.asm:70080`) and counted down by F938's whole-second events, which clear
  `+0x151` bits `0x60` at zero. The port draws no model 0xa1 and does not carry
  `+0x14e`/`+0x14f` for NPCs. Its RNG draws would also shift the shared stream.
  Needs a run of the original with a ship hit to see when it appears before porting.

## Takeoff climb drift: fixed (2026-11-17, parity-04)

`tests/takeoff_replay.rs` (original `--flight takeoff 900`, Ross 154) showed the
port's climb 1.2% slow in speed, its altitude 1% low and F1031's clear 2 frames
early (428, original 430). The cause was the frame: the original keeps the ship in
the planet's **rotating** frame (`+0x57`; here the planet's rows are
diag(-1, 1, -1)), while the port's pad world integrated it in the planet-linear
frame, whose x and z signs are flipped. F601 floors
`(row >> 16) * (tics >> 3) >> 15` per axis, and flooring a negative row differs by
one unit from flooring its negation, so each frame the port added x -11.6757 and
z +0.9979 where the original adds -11.7755 and +0.8981 (0.0998 each; the original's
climb velocity is exactly F601's thrust-only integration, and gravity was not the
cause).

Fixed: `OriginalFlight::begin_pad_takeoff` now turns the ship and the starport into
the planet's rotating frame once (F572), `step_pad` integrates there (thrust,
gravity, the fraction accumulators, the collision scan), and the linear view the
rest of the port reads (`player.motion`, `absolute_position`) is derived from that
state after every frame (`pad_state`/`pad_publish`); `move_pad_bodies` only moves
the planet, so a parked ship turns with it without a carry. The climb's velocity
now equals the original's to within 1 unit per axis through frame 430 and the clear
is one frame early (429; the altitude at the clear is within 0.1%). The replay test
asserts both.

Free flight after the clear (frame 430 on) was up to 1.25% slow (velocity z 3974
against 4018 at frame 500), and that was **not** the frame: `step_scan` ran F1520
on the ship's rows after every frame to take out its landing margin, and the
port's F1519 normalises to 0.992 of unit length (the constant `0x3f800000`), so
the rows it rebuilt came out at 0.985 and the thrust fell 1.5% from the first
frame after the clear (the thrust words were 5700 against the original's 5844
at frame 431). The original only runs F1520 when F611's drift counter goes
negative, which the tick already does. `step_scan` now divides the margin out
instead. The replay's velocity is within 1 unit per axis through frame 899 (z
20370 at frame 899 in both) and the largest speed difference is 0.18%; the test
asserts under 0.5%. The ship still returns to the system frame at the clear while
the original keeps it a child of planet 111; with this fix that has no visible
effect on the speed.

`tools/reference/headless.c` prints, on stderr only, the player's thrust words,
limits and rows for frames 161-176, 200 and 260 when `FFE_PLAYER_DUMP` is set (the
stdout trace and its hashes are unchanged).
