//! A row on a screen reached by navigation must be clickable where it is drawn.
//!
//! A screen transition clips the incoming screen to a sliver on its first
//! frame and replays its cached picture on every frame after. Replay
//! translates what the subtree declared but never re-declares it, and
//! `register_hit` intersected those rects with the clip in force at
//! declaration time — so the rows stayed clickable only in their leftmost
//! ~50px while rendering full width.
//!
//! Reported as "clicking is scrolling": a tap on the dead part of a row fell
//! through to the scrollable underneath, so tapping a list item scrolled the
//! list instead of opening it.
//!
//! This lives in the showcase rather than in `rosace/tests` deliberately. The
//! defect needs a real `ScreenNav` push through a real `ScreenTransitionView`
//! inside a real `Scaffold`; every reduced fixture re-recorded the subtree
//! somewhere in the transition and quietly proved nothing.

use rosace::prelude::{InputEvent, MouseButton};
use rosace::{FontCache, FrameEngine, SkiaCanvas};
use showcase::AppRoot;

const W: u32 = 420;
const H: u32 = 820;

struct App { e: FrameEngine, a: SkiaCanvas, b: SkiaCanvas }
impl App {
    fn new() -> Self {
        App { e: FrameEngine::new(Box::new(AppRoot), FontCache::embedded()),
              a: SkiaCanvas::new(W, H), b: SkiaCanvas::new(W, H) }
    }
    fn ev(&mut self, v: &[InputEvent]) { self.e.paint(&mut self.a, &mut self.b, v); }
    fn settle(&mut self) { for _ in 0..60 { self.ev(&[]); } }
    /// The welcome screen plays an intro before it offers its action.
    fn intro(&mut self) { for _ in 0..240 { self.ev(&[]); } }
    fn tap(&mut self, x: f32, y: f32) {
        self.ev(&[InputEvent::MouseDown { x, y, button: MouseButton::Left },
                  InputEvent::MouseUp   { x, y, button: MouseButton::Left }]);
    }
    /// Screen identity, by the labels assistive tech would read.
    fn labels(&self) -> Vec<String> {
        let mut v: Vec<String> = self.e.inspect_tree().iter()
            .flat_map(|n| n.semantics.iter().filter_map(|(_, l)| l.clone()))
            .collect();
        v.sort();
        v
    }
    /// Every row on the current screen that declares a hit region.
    fn rows(&self) -> Vec<rosace::prelude::Rect> {
        self.e.inspect_tree().iter()
            .filter(|n| n.tag.ends_with("ListTile") && n.hit_count > 0)
            .filter_map(|n| n.rect)
            .collect()
    }
    /// Walk to the catalog, which is reached by a push and so arrives
    /// through a transition.
    fn goto_catalog(&mut self) {
        self.intro();
        let btn = self.e.inspect_tree().iter()
            .filter(|n| n.tag.ends_with("Button") && n.hit_count > 0)
            .filter_map(|n| n.rect)
            .next()
            .expect("the welcome screen offers an action");
        self.tap(btn.origin.x + btn.size.width / 2.0, btn.origin.y + btn.size.height / 2.0);
        self.settle();
    }
}

/// Tapping the middle of a row must do something. Asserted across the row's
/// width: the bug left only a narrow strip at the left edge alive, so a
/// centre-of-row tap — where anyone would actually press — was dead.
#[test]
fn a_row_is_clickable_across_its_full_width_after_navigating_to_its_screen() {
    let mut app = App::new();
    app.goto_catalog();
    let rows = app.rows();
    assert!(!rows.is_empty(), "the catalog painted no rows to tap");

    let row = rows[0];
    let y = row.origin.y + row.size.height / 2.0;
    // Three quarters across the row, far outside the transition's opening
    // sliver and unambiguously inside the row as drawn.
    let x = row.origin.x + row.size.width * 0.75;

    let before = app.labels();
    app.tap(x, y);
    app.settle();

    assert_ne!(
        app.labels(), before,
        "tapping the row at ({x:.0}, {y:.0}) did nothing, though it is drawn from \
         x={:.0} to x={:.0}. Its hit region is still clipped to the sliver the \
         screen transition opened with, so the tap fell through to whatever is \
         underneath.",
        row.origin.x, row.origin.x + row.size.width,
    );
}
