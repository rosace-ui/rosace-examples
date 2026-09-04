//! showcase — a ROSACE app.
//!
//! `launch()` is shared by every platform. The native binary calls it from
//! `main`; the web build calls it from a `wasm-bindgen(start)` entry.

mod app;

/// The real application root, exported so it can be driven HEADLESSLY.
///
/// A showcase that merely compiles and launches proves very little: most of
/// what broke during the engine refactor was invisible to the compiler and
/// visible only on interaction — clicks landing on the wrong widget, hover
/// sticking, a dialog that could not be dismissed. Constructing this against a
/// `FrameEngine` lets a test navigate the real screens and click the real
/// widgets with no window.
pub use app::{AppRoot, Screen, WidgetKind};
mod feedback;
mod present;
mod ffi;
mod screens;
mod theme;

/// Typed handles for everything under `assets/`, generated at build time by
/// `build.rs`. Refer to assets as `assets::LOGO` (typo-proof, autocompletes)
/// rather than raw strings. Add a file to `assets/` and it appears here.
pub mod assets {
    include!(concat!(env!("OUT_DIR"), "/rosace_assets.rs"));
}

use rosace::prelude::*;

/// The channel name for this app's own custom method (the "native calls
/// Rust, synchronously" direction — see `screens::platform_channel`'s doc
/// for the full picture). Channel names are just strings you pick — using
/// your bundle id as a prefix (like a Java package) avoids collisions with
/// other libraries' channels in the same app.
pub const MATH_CHANNEL: &str = "dev.rosace.showcase/math";

/// One-time app startup — register Platform Channel method handlers here
/// (`rosace_ffi::set_method_call_handler`), or anything else that must run
/// exactly once before the engine starts.
///
/// Called from EVERY entry point below (`launch`, and — on iOS/Android —
/// `ffi.rs`'s `rsc_engine_init`/`nativeInit`), not just this one: mobile's
/// FFI entry points construct the engine directly and never call `launch`,
/// so code that only ran here would silently never execute on iOS/Android
/// (found live: a Platform Channel handler registered only in `launch`
/// answered every call with "no handler registered" on mobile until its
/// registration moved here instead).
pub(crate) fn app_init() {
    // So system-brightness switching (`rosace_theme::sync_system_theme`,
    // driven by the native OS-appearance push) applies THIS app's themes
    // instead of the framework's generic built-in light/dark — otherwise a
    // customized `theme.rs` would silently stop mattering the moment the OS
    // flips dark mode.
    rosace::theme::register_theme_pair(theme::light(), theme::dark());

    // Registers the starter shader material library (`gradient`/`noise`/
    // `glow`) so the ShaderPaint/Container/AppBar "shader material" demo
    // pages in the widget catalog resolve their pipelines.
    materials::register_starter_materials();

    rosace_ffi::set_method_call_handler(MATH_CHANNEL, Box::new(|method, args| {
        match method {
            "add" => {
                let nums: Vec<i64> = serde_json::from_value(args)
                    .map_err(|e| format!("expected a JSON array of numbers: {e}"))?;
                Ok(serde_json::Value::from(nums.iter().sum::<i64>()))
            }
            other => Err(format!("unknown method '{other}' on {MATH_CHANNEL}")),
        }
    }));
}

/// Start the app. Runs the winit event loop on native; hands off to the
/// browser's requestAnimationFrame loop on web.
pub fn launch() {
    app_init();
    // Window size applies on desktop; mobile is always fullscreen.
    App::new()
        .title("showcase")
        .size(960, 640)
        .themes(theme::themes())
        .launch(app::AppRoot);
}

/// Web (wasm) entry — invoked automatically when the module is instantiated.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    launch();
}

#[cfg(test)]
mod catalog_tests {
    //! Every catalog page is painted headlessly here.
    //!
    //! Compiling proves nothing about a demo screen: a page that panics on
    //! its first paint, or lays out to nothing, or silently paints an empty
    //! rect, compiles perfectly. This walks `WidgetKind::ALL`, so a widget
    //! added to the catalog is covered the moment it is listed — there is no
    //! second place to remember to update.

    use crate::app::{WidgetDemoState, WidgetKind};
    use rosace::prelude::*;
    use rosace::FrameEngine;
    use rosace::{FontCache, SkiaCanvas};

    /// Renders one detail page. `WidgetDemoState` needs a real `Context` to
    /// allocate its atoms, so the page has to be built inside a `Component`.
    struct Page(WidgetKind);

    impl Component for Page {
        fn build(&self, ctx: &mut Context) -> BoxedWidget {
            let demo = WidgetDemoState::new(ctx);
            // A real navigator: the WillPopScope page pops through it, and
            // ScreenNav registers the back handler on construction.
            let nav = rosace::nav::ScreenNav::new(ctx, crate::app::Screen::Widgets);
            crate::screens::widgets::widget_detail_screen(self.0, &demo, &nav).boxed()
        }
    }

    /// Paints twice. The second frame is what catches a page that only works
    /// once — `Responsive` runs its builder on both the layout and the paint
    /// pass, so a page that hands its content over instead of rebuilding it
    /// renders the first frame and then goes blank.
    fn painted_pixels(kind: WidgetKind) -> usize {
        let mut e = FrameEngine::new(Box::new(Page(kind)), FontCache::embedded());
        let (mut c, mut o) = (SkiaCanvas::new(420, 800), SkiaCanvas::new(420, 800));
        e.paint(&mut c, &mut o, &[]);
        e.paint(&mut c, &mut o, &[]);
        // Count pixels that are not fully transparent.
        c.pixels().chunks_exact(4).filter(|px| px[3] != 0).count()
    }

    #[test]
    fn every_catalog_page_paints_something() {
        let mut blank = Vec::new();
        for kind in WidgetKind::ALL {
            let n = painted_pixels(*kind);
            // A real page fills a 420x800 viewport with a background at
            // minimum; a few hundred pixels means it laid out to nothing.
            if n < 10_000 {
                blank.push(format!("  {} painted only {n} pixels", kind.name()));
            }
        }
        assert!(blank.is_empty(), "catalog pages that render (almost) nothing:\n{}", blank.join("\n"));
    }

    #[test]
    fn every_catalog_page_announces_something() {
        // Quality Bar §5 at the page level: a screen that declares no
        // semantics at all is unusable with a screen reader.
        let mut silent = Vec::new();
        for kind in WidgetKind::ALL {
            let mut e = FrameEngine::new(Box::new(Page(*kind)), FontCache::embedded());
            let (mut c, mut o) = (SkiaCanvas::new(420, 800), SkiaCanvas::new(420, 800));
            e.paint(&mut c, &mut o, &[]);
            fn count(n: &rosace::SemanticNode) -> usize {
                (if n.role != Role::Unknown || n.label.is_some() { 1 } else { 0 })
                    + n.children.iter().map(count).sum::<usize>()
            }
            if count(&e.semantics()) == 0 {
                silent.push(format!("  {}", kind.name()));
            }
        }
        assert!(silent.is_empty(), "catalog pages with no semantics:\n{}", silent.join("\n"));
    }

    /// The WillPopScope demo must actually GUARD, not just render.
    ///
    /// A catalog page that paints is not necessarily wired — this one's
    /// whole point is the guard, and forgetting `.on_will_pop` would still
    /// look perfect on screen. So this drives the real thing: dirty the
    /// draft, attempt the pop every exit route goes through, and assert it
    /// was refused and the confirmation opened.
    #[test]
    fn the_will_pop_scope_demo_blocks_leaving_with_unsaved_work() {
        use rosace::nav::ScreenNav;
        use std::sync::{Arc, Mutex};

        struct Page {
            draft: Arc<Mutex<Option<Atom<String>>>>,
            confirm: Arc<Mutex<Option<Atom<bool>>>>,
            nav: Arc<Mutex<Option<ScreenNav<crate::app::Screen>>>>,
        }
        impl Component for Page {
            fn build(&self, ctx: &mut Context) -> BoxedWidget {
                let demo = WidgetDemoState::new(ctx);
                let nav = ScreenNav::new(ctx, crate::app::Screen::Widgets);
                // Somewhere to pop back to, seeded once.
                let seeded = ctx.state(false);
                if !seeded.get() {
                    seeded.set(true);
                    nav.push(crate::app::Screen::WidgetDetail(WidgetKind::WillPopScope));
                }
                *self.draft.lock().unwrap() = Some(demo.will_pop_draft.clone());
                *self.confirm.lock().unwrap() = Some(demo.will_pop_confirm.clone());
                *self.nav.lock().unwrap() = Some(nav.clone());
                crate::screens::widgets::widget_detail_screen(
                    WidgetKind::WillPopScope, &demo, &nav,
                ).boxed()
            }
        }

        let (draft, confirm, nav_out) = (
            Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)),
        );
        let mut e = FrameEngine::new(
            Box::new(Page { draft: draft.clone(), confirm: confirm.clone(), nav: nav_out.clone() }),
            FontCache::embedded(),
        );
        let (mut c, mut o) = (SkiaCanvas::new(420, 800), SkiaCanvas::new(420, 800));
        e.paint(&mut c, &mut o, &[]);

        let draft = draft.lock().unwrap().clone().unwrap();
        let confirm = confirm.lock().unwrap().clone().unwrap();
        let nav = nav_out.lock().unwrap().clone().unwrap();
        assert_eq!(nav.depth(), 2, "the demo page is pushed");

        // Clean: leaving is allowed.
        assert!(nav.can_pop());
        assert!(!confirm.get(), "no question asked yet");

        // Dirty it, repaint so the guard re-registers with the new state.
        draft.set("unsaved words".into());
        e.paint(&mut c, &mut o, &[]);

        assert!(!nav.pop(), "the guard must refuse the pop");
        e.paint(&mut c, &mut o, &[]);
        assert!(confirm.get(), "the guard must open the confirmation");
        assert_eq!(nav.depth(), 2, "still on the demo page");
    }

}
