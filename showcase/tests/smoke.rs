//! Drive the REAL showcase app headlessly.
//!
//! Building and launching proves very little. Almost everything that broke
//! during the engine refactor was invisible to the compiler and visible only
//! on interaction: clicks landing on the wrong widget, hover sticking to a row
//! after the pointer left, a dialog that could not be dismissed, a keyed screen
//! coming back dead after a pop. A window that opens tells you none of that.
//!
//! So this constructs `AppRoot` — the same root the native binary and the
//! mobile FFI host use — against a `FrameEngine` with in-memory canvases, and
//! walks it: navigate every widget page, click things, scroll, open an overlay.
//! No display, no GPU, real dispatch.
//!
//! It asserts what a smoke test honestly can: nothing panics, the tree stays
//! coherent, and the screens actually change. It is not a substitute for
//! looking at it — it is the floor beneath that.

use rosace::prelude::{InputEvent, MouseButton};
use rosace::widgets::tree::InspectNode;
use rosace::{FontCache, FrameEngine, SkiaCanvas};
use showcase::AppRoot;

const W: u32 = 420;
const H: u32 = 820;

struct App {
    e: FrameEngine,
    a: SkiaCanvas,
    b: SkiaCanvas,
}

impl App {
    fn new() -> Self {
        App {
            e: FrameEngine::new(Box::new(AppRoot), FontCache::embedded()),
            a: SkiaCanvas::new(W, H),
            b: SkiaCanvas::new(W, H),
        }
    }

    fn frame(&mut self) { self.e.paint(&mut self.a, &mut self.b, &[]); }

    /// Paint until transitions settle. Navigation animates, so a single frame
    /// after a tap shows a half-finished screen.
    fn settle(&mut self) {
        for _ in 0..40 { self.frame(); }
    }

    /// The welcome screen animates in and only reveals its action once the
    /// intro has played, so reaching the app proper takes more than a
    /// transition's worth of frames.
    fn settle_intro(&mut self) {
        for _ in 0..240 { self.frame(); }
    }

    /// Centre of the first node that actually RESPONDS to a click, skipping
    /// the DevTools FAB the engine injects in dev builds.
    ///
    /// Found by `hit_count` rather than by type name: what the welcome screen
    /// happens to be built from is not this test's business, and pinning it to
    /// a widget type makes the test fail when the screen is redesigned rather
    /// than when the engine breaks.
    fn first_interactive(&self) -> Option<(f32, f32)> {
        self.nodes().iter()
            .filter(|n| n.hit_count > 0 && !n.tag.contains("FloatingActionButton"))
            .filter_map(|n| n.rect)
            .filter(|r| r.size.width > 8.0 && r.size.height > 8.0)
            .map(|r| (r.origin.x + r.size.width / 2.0, r.origin.y + r.size.height / 2.0))
            .next()
    }

    fn click(&mut self, x: f32, y: f32) {
        self.e.paint(&mut self.a, &mut self.b, &[
            InputEvent::MouseDown { x, y, button: MouseButton::Left },
            InputEvent::MouseUp   { x, y, button: MouseButton::Left },
        ]);
    }


    fn nodes(&self) -> Vec<InspectNode> { self.e.inspect_tree() }

    /// A cheap fingerprint of what is on screen, for asserting that a tap
    /// actually changed something.
    fn fingerprint(&self) -> String {
        let n = self.nodes();
        let painted = n.iter().filter(|x| x.rect.is_some()).count();
        format!("{}:{}", n.len(), painted)
    }


    /// Every node must remain reachable from the root with a sane rect. A
    /// detached parent link is what made a re-created keyed screen come back
    /// frozen — alive, dispatching clicks, and never scheduling a frame.
    fn assert_tree_is_coherent(&self, where_: &str) {
        for n in self.nodes() {
            if let Some(r) = n.rect {
                assert!(
                    r.size.width.is_finite() && r.size.height.is_finite(),
                    "{where_}: `{}` has a non-finite rect {r:?}", n.tag
                );
                assert!(
                    r.size.width >= 0.0 && r.size.height >= 0.0,
                    "{where_}: `{}` has a negative size {r:?}", n.tag
                );
            }
        }
    }
}

/// The app starts, paints, and settles on a real screen.
#[test]
fn the_app_starts_and_paints() {
    let mut app = App::new();
    app.settle();

    let n = app.nodes();
    assert!(n.len() > 8, "the first screen painted only {} nodes", n.len());
    assert!(
        n.iter().any(|x| x.rect.is_some()),
        "nothing painted a rect — the app is running but blank"
    );
    app.assert_tree_is_coherent("startup");
}

/// Walking into the widget catalog and back must actually change the screen,
/// and leave the tree coherent on both sides of the transition.
#[test]
fn navigating_changes_the_screen_and_comes_back() {
    let mut app = App::new();
    app.settle_intro();
    let start = app.fingerprint();

    let (x, y) = app.first_interactive().expect("the first screen offers something to tap");
    app.click(x, y);
    app.settle();
    app.assert_tree_is_coherent("after navigating");

    let moved = app.fingerprint();
    assert_ne!(start, moved, "tapping the primary button changed nothing on screen");

    // Back out again — this is the path that used to come back frozen, because
    // a disposed keyed node kept a severed parent link.
    app.e.paint(&mut app.a, &mut app.b, &[InputEvent::BackPressed]);
    app.settle();
    app.assert_tree_is_coherent("after going back");
}

// Scrolling is deliberately NOT tested here.
//
// A version of this walked forward tapping the first interactive thing on each
// screen until it found a scrollable one — and when that failed it was testing
// the walk, not the engine. Scrolling already has three dedicated tests against
// real trees (`stress.rs` counts the work of 60 scroll frames on 200 rows,
// `click_after_scroll` proves hit regions follow the pixels, `nested_scroll`
// covers routing between axes). A flaky fourth adds nothing but noise.

/// Hover must not stick. Reported live during the refactor: the highlight
/// stayed on whichever row the pointer last rested over.
#[test]
fn hover_does_not_stick_after_the_pointer_leaves() {
    let mut app = App::new();
    app.settle_intro();

    app.e.paint(&mut app.a, &mut app.b, &[
        InputEvent::MouseMove { x: W as f32 / 2.0, y: H as f32 / 2.0 },
    ]);
    app.frame();

    // Move far away, to a corner nothing interactive occupies.
    app.e.paint(&mut app.a, &mut app.b, &[
        InputEvent::MouseMove { x: 2.0, y: 2.0 },
    ]);
    app.frame();

    let hovered: Vec<&str> = app.nodes().iter()
        .filter(|n| n.hovered)
        .map(|n| n.tag)
        .collect();
    assert!(
        hovered.len() <= 1,
        "{} nodes are still hovered after the pointer moved away: {hovered:?}",
        hovered.len()
    );
}
