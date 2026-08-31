//! A `.hero_tag`'d element must MORPH between screens, not teleport.
//!
//! `Hero::paint` has a side effect: while a transition is in flight it
//! captures itself into the hero registry instead of painting in place, and
//! `ScreenTransitionView` pairs the two sides and paints one floating copy
//! lerp'd between their rects.
//!
//! Replay-on-move reproduces pixels and skips paint — so it skips the
//! capture. That is exactly the OUTGOING screen, which by definition has
//! painted before: its Hero never registered, no pair matched, and BOTH
//! captures were dropped. The element vanished for the whole transition and
//! reappeared at the destination.
//!
//! Asserted by scanning pixels for the swatch colour, because the flight is
//! recorded into the transition view's own picture rather than a node — there
//! is no rect to inspect. The failure signature is specific: the swatch is
//! absent from every mid-transition frame.

use rosace::prelude::{InputEvent, MouseButton, Rect};
use rosace::{FontCache, FrameEngine, SkiaCanvas};
use showcase::AppRoot;

const W: u32 = 420;
const H: u32 = 820;
const SWATCH: (u8, u8, u8) = (86, 118, 220);   // SWATCHES[0] in hero.rs

struct App { e: FrameEngine, a: SkiaCanvas, b: SkiaCanvas }
impl App {
    fn new() -> Self {
        App { e: FrameEngine::new(Box::new(AppRoot), FontCache::embedded()),
              a: SkiaCanvas::new(W, H), b: SkiaCanvas::new(W, H) }
    }
    fn ev(&mut self, v: &[InputEvent]) { self.e.paint(&mut self.a, &mut self.b, v); }
    fn frame(&mut self) { self.ev(&[]); }
    fn settle(&mut self) { for _ in 0..60 { self.frame(); } }
    fn intro(&mut self) { for _ in 0..240 { self.frame(); } }
    fn tap(&mut self, x: f32, y: f32) {
        self.ev(&[InputEvent::MouseDown { x, y, button: MouseButton::Left }]);
        for _ in 0..3 { self.frame(); }
        self.ev(&[InputEvent::MouseUp { x, y, button: MouseButton::Left }]);
    }
    fn labels(&self) -> Vec<String> {
        self.e.inspect_tree().iter()
            .flat_map(|n| n.semantics.iter().filter_map(|(_, l)| l.clone())).collect()
    }
    /// Bring the row into view with the engine's own `reveal`, then tap the
    /// centre of the viewport where it now sits. Row rects are content-space
    /// under a composited ScrollView, so they cannot be used directly.
    fn tap_label(&mut self, want: &str) -> bool {
        let id = self.e.inspect_tree().iter()
            .filter(|n| n.hit_count > 0)
            .find(|n| n.semantics.iter().any(|(_, l)|
                l.as_deref().map_or(false, |s| s.starts_with(want))))
            .map(|n| n.id);
        let Some(id) = id else { return false };
        let rect = self.e.inspect_tree().iter().find(|n| n.id == id).and_then(|n| n.rect);
        // Already on screen (a short, uncomposited list): tap it where it is.
        if let Some(r) = rect {
            if r.origin.y > 50.0 && r.origin.y + r.size.height < H as f32 - 50.0 {
                self.tap(r.origin.x + r.size.width / 2.0, r.origin.y + r.size.height / 2.0);
                self.settle();
                return true;
            }
        }
        // Otherwise it is off-screen in a composited list, where its rect is
        // content-space and unusable — let the engine bring it to the middle.
        self.e.reveal(id, rosace::scroll::ScrollAlign::Center);
        self.settle();
        self.tap(W as f32 / 2.0, H as f32 / 2.0);
        self.settle();
        true
    }

    /// Bounding box of the swatch colour in the CURRENT front buffer.
    fn swatch_box(&self) -> Option<Rect> {
        self.box_in(&self.a).or_else(|| self.box_in(&self.b))
    }
    fn box_in(&self, c: &SkiaCanvas) -> Option<Rect> {
        let px = c.pixels();
        let (w, h) = (c.width(), c.height());
        let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0u32, 0u32);
        for y in 0..h {
            for x in 0..w {
                let i = ((y * w + x) * 4) as usize;
                if i + 2 < px.len()
                    && px[i].abs_diff(SWATCH.0) < 6
                    && px[i+1].abs_diff(SWATCH.1) < 6
                    && px[i+2].abs_diff(SWATCH.2) < 6
                {
                    x0 = x0.min(x); y0 = y0.min(y); x1 = x1.max(x); y1 = y1.max(y);
                }
            }
        }
        if x0 == u32::MAX { None } else {
            Some(Rect { origin: rosace::prelude::Point { x: x0 as f32, y: y0 as f32 },
                        size: rosace::prelude::Size { width: (x1 - x0) as f32, height: (y1 - y0) as f32 } })
        }
    }
}

#[test]
fn the_hero_element_actually_flies() {
    let mut app = App::new();
    app.intro();
    // Welcome -> Home
    let b = app.e.inspect_tree().iter()
        .filter(|n| n.tag.ends_with("Button") && n.hit_count > 0)
        .filter_map(|n| n.rect).next().unwrap();
    app.tap(b.origin.x + b.size.width / 2.0, b.origin.y + b.size.height / 2.0);
    app.settle();
    assert!(app.tap_label("Widgets"), "no Widgets row");
    app.settle();
    assert!(app.tap_label("Hero"), "no Hero row; labels: {:?}", app.labels());
    app.settle();

    if app.swatch_box().is_none() {
        eprintln!("labels on this page: {:?}", app.labels());
        let mut seen = std::collections::BTreeSet::new();
        for c in [&app.a, &app.b] {
            let px = c.pixels();
            for i in (0..px.len()).step_by(4 * 97) {
                if i + 2 < px.len() { seen.insert((px[i], px[i+1], px[i+2])); }
            }
        }
        eprintln!("sample colours present: {:?}", seen.iter().take(30).collect::<Vec<_>>());
    }
    let start = app.swatch_box().expect("the hero page shows swatch 0");
    eprintln!("SWATCH at rest: {start:?}");

    // Tap the swatch itself to push the detail screen.
    app.tap(start.origin.x + start.size.width / 2.0, start.origin.y + start.size.height / 2.0);

    // Watch it across the transition.
    let mut seen: Vec<(f32, f32, f32)> = vec![];
    for _ in 0..24 {
        app.frame();
        if let Some(r) = app.swatch_box() {
            seen.push((r.origin.x, r.origin.y, r.size.width));
        } else {
            seen.push((-1.0, -1.0, -1.0));
        }
    }
    eprintln!("SWATCH during transition (x, y, w):");
    for (i, s) in seen.iter().enumerate() { eprintln!("  frame {i:>2}: {s:?}"); }

    let widths: Vec<f32> = seen.iter().map(|s| s.2).filter(|w| *w > 0.0).collect();
    let distinct = widths.iter().map(|w| (*w as i32 / 8) * 8).collect::<std::collections::BTreeSet<_>>();
    eprintln!("distinct widths (8px buckets): {distinct:?}");
    assert!(
        distinct.len() >= 3,
        "the hero never morphed: its width took {} distinct value(s) across the \
         whole transition. It should grow continuously from the 64px tile to the \
         280px detail image.",
        distinct.len()
    );
}
