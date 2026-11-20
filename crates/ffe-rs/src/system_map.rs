//! The original's 3D system map: module 5 of `DATA_007239` (F963-F986 in
//! `jjffe-master/ffe.asm`), the page with the "Time point:" line, the orbit
//! curves and the body labels. The galaxy map reaches it with F6 (this system,
//! F966) or `n`/`r` (the selected star's system, F974); its F5 and F7-F10 hand
//! back to the galaxy map's pages. Docs: `docs/notes/system-map.md`.
//!
//! What is the original's here: the page's keys and hot areas (F982), the time
//! point and the slider that moves it (F980: the offset from the slider's
//! centre, squared for this system and cubed for another, shifted left 18 into
//! the 2^32-per-day clock), the orbit of every object that has a parent (F986),
//! the name labels that skip starports and the view's right and bottom edges
//! (F975/F978), the target marker, and the click that targets an object within
//! eight pixels of its label point (F983). What is the port's own: the projection
//! is a plain rotation and scale rather than F1537's scene, so there is no
//! perspective, and the orbit is the circle F1507 places the object on, drawn
//! as a polyline instead of F986's ring model. See `docs/DEVIATIONS.md`.

use crate::navigation::{Body, Kind};

/// The page's drawing height: the cabin view above the console (F977 clips at
/// y `0x95`).
pub const VIEW_HEIGHT: i32 = 158;
/// `DATA_007241`'s hot area (x0, y0, x1, y1): F982's action `0x80` slider.
pub const SLIDER: (i32, i32, i32, i32) = (268, 156, 319, 177);
/// F982 measures the slider's click from this x (`0x12c`).
const SLIDER_CENTRE: i32 = 0x12c;
/// One astronomical unit in millimetres, for the details.
const MM_PER_AU: f64 = 1.495_978_707e14;
/// Seconds in a game day (the clock's high dword counts days).
const DAY: f64 = 86_400.0;
/// The view's centre.
const CENTRE: (f64, f64) = (160.0, 79.0);
/// F983's pick distance (the sum of the pixel offsets, `cmp edx,8`).
const PICK: i32 = 8;

/// The text colours F842 uses: green orbit lines, yellow, white.
pub const GREEN: u8 = 17;
pub const YELLOW: u8 = 18;
pub const WHITE: u8 = 15;

/// One system seen from outside, at a time point.
#[derive(Debug, Clone)]
pub struct SystemMap {
    /// The bodies at [`Self::clock`], in the order of the navigator's list.
    pub bodies: Vec<Body>,
    /// The system's name, for the details.
    pub system: String,
    /// The player's own system (F966, slider squared) rather than the star the
    /// galaxy map selected (F974, slider cubed).
    pub home: bool,
    /// The game clock when the page opened, in world seconds.
    pub now: f64,
    /// The time point: the clock the bodies stand at.
    pub clock: f64,
    /// The turn about the system's up axis and the tilt down toward it, radians.
    pub yaw: f64,
    pub pitch: f64,
    /// Pixels to a millimetre.
    pub scale: f64,
    /// The body at the view's centre.
    pub focus: usize,
    /// The body the last click picked (F983's target).
    pub selected: Option<usize>,
    /// `t`: the next click is a destination for the autopilot (F982's flag).
    pub navcomp: bool,
}

/// A body's dot as drawn.
#[derive(Debug, Clone, PartialEq)]
pub struct Dot {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub colour: u8,
    pub kind: Kind,
    pub depth: f64,
    /// The picked body (F978's marker is drawn about it).
    pub marked: bool,
}

/// A name written beside a dot (F978).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub x: i32,
    pub y: i32,
    pub text: String,
    /// The picked body, whose label carries F978's marker.
    pub marked: bool,
}

/// Everything the screen draws for one frame of the page.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Frame {
    pub orbits: Vec<Vec<(i32, i32)>>,
    pub dots: Vec<Dot>,
    pub labels: Vec<Label>,
    /// "Time point: ..." (F977's string 0, white at (10, 148)).
    pub time_point: String,
    /// The picked body's facts, bottom up from y 137.
    pub details: Vec<String>,
}

fn sub(a: [i64; 3], b: [i64; 3]) -> [f64; 3] {
    [
        (a[0] - b[0]) as f64,
        (a[1] - b[1]) as f64,
        (a[2] - b[2]) as f64,
    ]
}

fn length(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// F982's slider (action `0x80`): the game seconds one click at logical x moves
/// the time point; negative left of the slider's centre.
pub fn slider_step(x: i32, home: bool) -> f64 {
    let offset = x - SLIDER_CENTRE;
    let magnitude = i64::from(offset.abs());
    let value = if home {
        magnitude * magnitude
    } else {
        magnitude * magnitude * magnitude
    };
    // `value << 18` in a clock whose low dword is 2^32 to the day.
    let days = value as f64 / 16_384.0;
    days.copysign(f64::from(offset)) * DAY
}

/// Whether a logical point is on the slider's hot area.
pub fn on_slider(x: i32, y: i32) -> bool {
    (SLIDER.0..=SLIDER.2).contains(&x) && (SLIDER.1..=SLIDER.3).contains(&y)
}

impl SystemMap {
    pub fn new(bodies: Vec<Body>, system: String, home: bool, clock: f64) -> Self {
        let focus = bodies
            .iter()
            .position(|body| body.kind == Kind::Star)
            .unwrap_or(0);
        let mut map = SystemMap {
            bodies,
            system,
            home,
            now: clock,
            clock,
            yaw: 0.0,
            pitch: 1.05,
            scale: 1e-12,
            focus,
            selected: None,
            navcomp: false,
        };
        map.fit();
        map
    }

    /// The farthest orbit about the focus: the planets of a star, the moons and
    /// stations of a world; `None` for a body with nothing around it.
    fn reach(&self) -> Option<f64> {
        let focus = self.bodies.get(self.focus)?;
        self.bodies
            .iter()
            .enumerate()
            .filter(|(i, body)| {
                *i != self.focus
                    && body.kind != Kind::Starport
                    && if focus.kind == Kind::Star {
                        body.kind == Kind::Planet
                    } else {
                        body.parent == Some(self.focus)
                    }
            })
            .map(|(_, body)| length(sub(body.position, focus.position)))
            .fold(None, |best: Option<f64>, r| {
                Some(best.map_or(r, |b| b.max(r)))
            })
    }

    /// Scale the page so the focus's farthest orbit fills most of the view.
    pub fn fit(&mut self) {
        let reach = self.reach().filter(|r| *r > 0.0).unwrap_or_else(|| {
            self.bodies
                .get(self.focus)
                .map_or(1e9, |body| body.radius as f64 * 4.0)
        });
        self.scale = 66.0 / reach;
    }

    /// Centre the page on a body (the scale is refitted to what orbits it).
    pub fn focus_on(&mut self, index: usize) {
        if index < self.bodies.len() {
            self.focus = index;
            self.fit();
        }
    }

    /// Back to the star (or the first body) the page opened on.
    pub fn reset_focus(&mut self) {
        let star = self
            .bodies
            .iter()
            .position(|body| body.kind == Kind::Star)
            .unwrap_or(0);
        self.focus_on(star);
    }

    /// Mouse turn (F982's accumulators): `dx` turns about the up axis, `dy` tilts.
    pub fn rotate(&mut self, dx: f64, dy: f64) {
        self.yaw += dx * 0.01;
        self.pitch = (self.pitch + dy * 0.01).clamp(0.0, std::f64::consts::FRAC_PI_2 + 0.3);
    }

    /// F11/F12 and Page Up/Down: multiply the scale.
    pub fn zoom(&mut self, factor: f64) {
        self.scale = (self.scale * factor).clamp(1e-20, 1e-3);
    }

    /// Replace the bodies for a new time point.
    pub fn set_time(&mut self, bodies: Vec<Body>, clock: f64) {
        if bodies.len() == self.bodies.len() {
            self.bodies = bodies;
        }
        self.clock = clock;
    }

    /// A world position on the page: the logical pixel and the depth along the view.
    pub fn project(&self, position: [i64; 3]) -> (f64, f64, f64) {
        let origin = self.bodies.get(self.focus).map_or([0; 3], |b| b.position);
        self.project_vector(sub(position, origin))
    }

    fn project_vector(&self, v: [f64; 3]) -> (f64, f64, f64) {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();
        let x = v[0] * cy + v[2] * sy;
        let turned = -v[0] * sy + v[2] * cy;
        let up = v[1] * cp + turned * sp;
        let depth = -v[1] * sp + turned * cp;
        (CENTRE.0 + self.scale * x, CENTRE.1 - self.scale * up, depth)
    }

    /// The orbit of a body as F1507 places it: a circle about its parent in the
    /// plane through the system's z axis and the body, as a polyline.
    fn orbit(&self, body: &Body) -> Option<Vec<(i32, i32)>> {
        if body.kind == Kind::Starport {
            return None;
        }
        let centre = body.orbit_centre?;
        let relative = sub(body.position, centre);
        let radius = length(relative);
        let pixels = radius * self.scale;
        if radius <= 0.0 || !(2.0..=40_000.0).contains(&pixels) {
            return None;
        }
        let flat = relative[0].hypot(relative[1]);
        let (hx, hy) = if flat > radius * 1e-9 {
            (relative[0] / flat, relative[1] / flat)
        } else {
            (1.0, 0.0)
        };
        let steps = (pixels * 0.9).clamp(24.0, 180.0) as usize;
        let origin = self.bodies.get(self.focus).map_or([0; 3], |b| b.position);
        let base = sub(centre, origin);
        let points = (0..=steps)
            .map(|k| {
                let angle = std::f64::consts::TAU * k as f64 / steps as f64;
                let (s, c) = angle.sin_cos();
                let v = [
                    base[0] + radius * hx * s,
                    base[1] + radius * hy * s,
                    base[2] + radius * c,
                ];
                let (x, y, _) = self.project_vector(v);
                (x.round() as i32, y.round() as i32)
            })
            .collect();
        Some(points)
    }

    /// The pixel a body's dot is drawn at and its radius (F978/F975 skip the
    /// starports and what lies past the view's right and bottom edges).
    fn dot(&self, index: usize) -> Option<Dot> {
        let body = self.bodies.get(index)?;
        if body.kind == Kind::Starport {
            return None;
        }
        let (x, y, depth) = self.project(body.position);
        let (x, y) = (x.round() as i32, y.round() as i32);
        let physical = (body.radius as f64 * self.scale).round() as i32;
        let (radius, colour) = match body.kind {
            Kind::Star => (physical.clamp(3, 12), YELLOW),
            Kind::Planet => (physical.clamp(2, 10), WHITE),
            Kind::Moon => (physical.clamp(1, 6), 11),
            _ => (1, GREEN),
        };
        Some(Dot {
            x,
            y,
            radius,
            colour,
            kind: body.kind,
            depth,
            marked: self.selected == Some(index),
        })
    }

    /// F983: the body whose label point is within eight pixels (the sum of the
    /// offsets) of a click, the nearest first.
    pub fn pick(&self, x: i32, y: i32) -> Option<usize> {
        (0..self.bodies.len())
            .filter_map(|i| {
                let dot = self.dot(i)?;
                let far = (dot.x - x).abs() + (dot.y - y).abs();
                (far <= PICK.max(dot.radius)).then_some((far, i))
            })
            .min()
            .map(|(_, i)| i)
    }

    /// What the page draws now. `date` is the time point's text; `range` the
    /// picked body's distance from the ship when it is known.
    pub fn frame(&self, date: &str, range: Option<String>) -> Frame {
        let mut frame = Frame {
            time_point: format!("Time point: {date}"),
            ..Frame::default()
        };
        for body in &self.bodies {
            if let Some(points) = self.orbit(body) {
                frame.orbits.push(points);
            }
        }
        let mut dots: Vec<Dot> = (0..self.bodies.len()).filter_map(|i| self.dot(i)).collect();
        dots.sort_by(|a, b| b.depth.total_cmp(&a.depth));
        frame.dots = dots;
        for (index, body) in self.bodies.iter().enumerate() {
            let Some(dot) = self.dot(index) else { continue };
            let marked = self.selected == Some(index);
            // A moon or station inside the ring of a few pixels would only
            // stack its name on its world's.
            let crowded = matches!(body.kind, Kind::Moon | Kind::Station)
                && !marked
                && body
                    .orbit_centre
                    .is_some_and(|c| length(sub(body.position, c)) * self.scale < 6.0);
            // F978: the label stands at the dot, and is dropped at the edges.
            if crowded || dot.x < 0 || dot.x + 3 >= 0x13c || dot.y - 4 < 0 || dot.y - 4 >= 0x95 {
                continue;
            }
            frame.labels.push(Label {
                x: dot.x + 6,
                y: dot.y - 4,
                text: body.name.clone(),
                marked,
            });
        }
        if let Some(body) = self.selected.and_then(|i| self.bodies.get(i)) {
            frame.details = self.details(body, range);
        }
        frame
    }

    fn details(&self, body: &Body, range: Option<String>) -> Vec<String> {
        let mut lines = vec![format!("{}: {}", body.name, body.kind.label())];
        if let Some(centre) = body.orbit_centre {
            let radius = length(sub(body.position, centre));
            if radius >= 1e10 {
                lines.push(format!("Orbit radius {:.3} A.U.", radius / MM_PER_AU));
            } else if radius > 0.0 {
                lines.push(format!("Orbit radius {:.0} km", radius / 1e6));
            }
        }
        if body.kind != Kind::Station && body.kind != Kind::Starport {
            lines.push(format!("Radius {:.0} km", body.radius as f64 / 1e6));
        }
        if let Some(range) = range {
            lines.push(format!("Range {range}"));
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body(kind: Kind, name: &str, position: [i64; 3], parent: Option<usize>) -> Body {
        Body {
            name: name.to_owned(),
            kind,
            system_type: 0,
            position,
            velocity: [0; 3],
            radius: 70_000_000_000,
            inner_radius: 1_000_000,
            parent,
            model: 0,
            seed: 0,
            mass: 0,
            slot: 0,
            rows: [0; 9],
            class: 0,
            open_air: false,
            orientation: [[0.0; 3]; 3],
            spin: 0,
            bays: 0,
            temperature: 0,
            size_word: 0,
            parent_frame: None,
            orbit_centre: parent.map(|_| [0; 3]),
        }
    }

    fn system() -> SystemMap {
        let au = MM_PER_AU as i64;
        SystemMap::new(
            vec![
                body(Kind::Star, "Sun", [0; 3], None),
                body(Kind::Planet, "Near", [au, 0, 0], Some(0)),
                body(Kind::Planet, "Far", [0, 0, 5 * au], Some(0)),
            ],
            "Test".to_owned(),
            true,
            0.0,
        )
    }

    #[test]
    fn the_focus_is_the_centre_of_the_view() {
        let map = system();
        let (x, y, _) = map.project([0; 3]);
        assert_eq!((x.round() as i32, y.round() as i32), (160, 79));
    }

    #[test]
    fn the_farthest_planet_fits_the_view() {
        let map = system();
        let dot = map.dot(2).expect("the far planet");
        let reach = (f64::from(dot.x) - 160.0).hypot(f64::from(dot.y) - 79.0);
        assert!((50.0..=80.0).contains(&reach), "{reach}");
    }

    #[test]
    fn a_click_within_eight_pixels_picks_the_nearest_body() {
        let map = system();
        let dot = map.dot(1).unwrap();
        assert_eq!(map.pick(dot.x + 2, dot.y + 3), Some(1));
        assert_eq!(map.pick(dot.x + 30, dot.y), None);
    }

    #[test]
    fn an_orbit_is_a_circle_about_its_parent() {
        let mut map = system();
        map.pitch = std::f64::consts::FRAC_PI_2;
        let orbit = map.orbit(&map.bodies[2]).expect("an orbit");
        let reach = 5.0 * MM_PER_AU * map.scale;
        for (x, y) in orbit {
            let r = (f64::from(x) - 160.0).hypot(f64::from(y) - 79.0);
            assert!((r - reach).abs() < 1.5, "{r} against {reach}");
        }
        // The star has no parent and so no orbit.
        assert!(map.orbit(&map.bodies[0]).is_none());
    }

    #[test]
    fn the_slider_squares_for_home_and_cubes_for_a_remote_system() {
        // 26 pixels left of the centre: 676 << 18 of a 2^32 day, about 59 minutes.
        let home = slider_step(SLIDER_CENTRE - 26, true);
        assert!((home + 676.0 / 16_384.0 * DAY).abs() < 1e-6, "{home}");
        let minutes = home.abs() / 60.0;
        assert!((minutes - 59.4).abs() < 0.1, "{minutes}");
        let remote = slider_step(SLIDER_CENTRE + 20, false);
        assert!((remote - 8_000.0 / 16_384.0 * DAY).abs() < 1e-6);
        assert!(on_slider(300, 160) && !on_slider(200, 160) && !on_slider(300, 120));
    }

    #[test]
    fn labels_skip_starports_and_the_view_edges() {
        let mut map = system();
        let frame = map.frame("1-Jan-3250 12:00:00", None);
        assert_eq!(frame.time_point, "Time point: 1-Jan-3250 12:00:00");
        assert!(frame.labels.iter().any(|l| l.text == "Sun"));
        map.scale *= 1000.0;
        let zoomed = map.frame("x", None);
        assert!(zoomed.labels.iter().all(|l| l.x < 0x13c && l.y < 0x95));
    }

    #[test]
    fn the_picked_body_has_details() {
        let mut map = system();
        map.selected = Some(1);
        let frame = map.frame("x", Some("1.0 A.U.".to_owned()));
        assert_eq!(frame.details[0], "Near: planet");
        assert!(frame
            .details
            .iter()
            .any(|l| l.starts_with("Orbit radius 1.000")));
        assert!(frame.details.iter().any(|l| l.starts_with("Range")));
    }
}
