# The 3D system map (module 5, F963-F986)

Added 2026-11-19 (`ui-29`, `ui-30`) after the owner's capture of the original's page: the
cabin with green orbit curves, white body names, a "Time point: 1-Jan-3250 11:02:00" line and
the console date. Code: `crates/ffe-rs/src/system_map.rs` (the page's model), `Screen::draw_system_map`,
and the input block in `main.rs` beside the galaxy map's.

## Where it sits in the original

`DATA_007239` is the event table of module 5 (`DATA_004892`'s sixth entry): F963 enter,
F966 re-enter this system, F982 keys and mouse, F977 draw, F985 time step, F964 leave. No
console key reaches module 5 directly; the galaxy map's handler (F840) does:

| Galaxy map input | Original | System map |
|---|---|---|
| **F6** (and the console's 0x23 icon), from any page of the map | `JUMP_003436` -> F966 | this system, time point = now (slider squared) |
| **`n`, `r`** with a star selected | `JUMP_003438` -> F974 | the selected star's system, generated for now (slider cubed) |

F982 hands F5-F10 back to the galaxy handler, so **F5 and F7-F10** close the page and act as they
do on the galaxy map (F5 controls page, F7 description, F8 sky, F9/F10 economy and people). F6 on
the page resets the time point and the focus.

## What the page does (F982, F975-F986)

- **Time point** (`DATA_007243`, F977's string 0, white at (10,148)): the date at the 64-bit time
  stored at `+0x400`. It starts at the game clock.
- **Slider** (action `0x80`, `DATA_007241`'s hot area (268,156)-(319,177)): a click at x moves the
  time point by `|x - 0x12c|^2` (this system) or `^3` (another) `<< 18` in a clock of 2^32 to the
  day, back when x is left of `0x12c`. 26 pixels left of the centre is 676 / 16384 of a day, 59 minutes:
  the capture's 11:02:00 against 12:00:14 on the console. Left/Right step it while held (the original's
  keys 3 and 4, `DATA_009122 << 13`, about 2.3 hours a second).
- **Orbits** (F986): every object with a parent, except starports (`+0x14c` bit 5), is drawn with
  its orbit about the parent. The bodies are F880/F866's for the time point (`Navigator::bodies_at`).
- **Names** (F975/F978): every object that is not a starport, white, at its dot; dropped when the
  dot is past x `0x13c` or y `0x95`. The picked body is marked.
- **Click** (F983): the body whose point is within 8 pixels (sum of the offsets) becomes the target
  (`Navigator::lock`). After **`t`** (F982's `0x74`, which logs "Click on a target or destination for
  autopilot", `DATA_007244`) the click engages the autopilot to it (`Navigator::engage_body`) and the
  map closes. Another star's system can be looked at but not flown to.
- **F11/F12** zoom in and out; the console's F11/F12 icons do the same.

## Port's own parts (see EXP-025 in `DEVIATIONS.md`)

- The projection is a rotation and a scale about the focus, not F1537's scene: no perspective.
  The right mouse button turns it (the original's mouse accumulators), Page Up/Down or +/- zoom
  smoothly.
- The orbit is the circle F1507 places the object on (a circle through the system's z axis, in the
  plane of the body and the axis), drawn as a polyline, where F986 draws a ring model.
- The original's focus (F984's object, DATA_008979) is the last object; the port focuses the star and
  fits its farthest planet. **C** (or a second click on a picked body) centres on that body and fits what
  orbits it; **Home** returns to the star.
- The picked body's facts (name, kind, orbit radius, radius, range) are written over the lower left of
  the view. The original shows only the label marker and, in flight, the target line.

## Not done

Perspective, F986's ring model, F984's initial zoom, the slider's pressed-arrow redraw
(`BlitClipIndexToBuf`), and the console message window for `t` going through F271's replace flag
exactly. Pixel comparison against the original's capture is by eye only.

## Tests

`system_map::tests` (projection, fit, pick, orbit circle, slider arithmetic, labels, details) and
`tests/system_map_view.rs` (local cache: Gateway's orbits, names and time point; the slider moves the
planets; a pick; `FFE_SHOT_DIR` writes the frames).
