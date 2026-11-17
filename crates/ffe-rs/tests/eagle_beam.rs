//! The Eagle's front-view laser no longer draws a fan. The executed original
//! leaves the 3D view unchanged: `jjffe-smoke --game-frame <n> 0x02` with `FFE_TAKEOFF=1
//! FFE_FIRE_AT=600` (the Eagle in flight, F931 on its front mount) differs from
//! the unfired run only in the message line (rows 184..198), never in the view
//! (rows 0..158). The port used to draw model 156 at its own scale 17 (512 times
//! the firing ship's scale 8, which F1776 inherits through F1533's copy of the
//! firing object) and showed a red fan; see `docs/KNOWN_ISSUES.md`.
//!
//! The port still draws a one-pixel ray (about 47 pixels from the lower right
//! towards the target) that the original does not: a recorded difference, so
//! the test only allows a thin ray, not a fan.
//!
//! `FFE_WORLD_CACHE=<data.bin> cargo test -p ffe-rs --test eagle_beam -- --ignored`

use ffe_rs::camera::ViewState;
use ffe_rs::combat;
use ffe_rs::flight::{Flight, Mode};
use ffe_rs::model_viewer::ModelViewer;
use ffe_rs::screen::Screen;

const VIEW_ROWS: usize = 158;
/// A one-pixel ray across the view (47 now); the old fan was thousands.
const MAX_RAY_PIXELS: usize = 120;

#[test]
#[ignore = "Local-only: needs the content cache (FFE_WORLD_CACHE)"]
fn the_eagles_front_beam_is_at_most_a_thin_ray() {
    let cache = ffe_content::read(&std::fs::read(std::env::var_os("FFE_WORLD_CACHE").unwrap()).unwrap())
        .unwrap();
    let viewer = ModelViewer::new(&cache).unwrap();
    let state = Flight {
        mode: Mode::Flight,
        system_flight: true,
        ship: 23,
        ..Flight::default()
    };
    let views = ViewState::default();
    let stats = viewer.ship_stats(state.ship);
    for laser in [0x88u8, 0xa0] {
        let mut screen = Screen::new(&cache).unwrap();
        screen.lasers = [laser, 0, 0, 0];
        screen.draw(&state, views, Some(&viewer), true, "");
        let before = screen.logical_pixels().to_vec();
        screen.beam = Some(combat::beam(&state, stats.as_ref(), views.mode, [0, 0]));
        screen.draw(&state, views, Some(&viewer), true, "");
        let changed = before
            .iter()
            .zip(screen.logical_pixels())
            .take(VIEW_ROWS * 320)
            .filter(|(a, b)| a != b)
            .count();
        assert!(
            changed <= MAX_RAY_PIXELS,
            "laser {laser:#x}: the Eagle's beam drew {changed} view pixels (a fan?)"
        );
    }
}
