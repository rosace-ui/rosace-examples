//! gallery_all — a ROSACE app.
//!
//! `launch()` is shared by every platform. The native binary calls it from
//! `main`; the web build calls it from a `wasm-bindgen(start)` entry.

pub mod app;
mod ffi;
pub mod theme;

/// Typed handles for everything under `assets/`, generated at build time by
/// `build.rs`. Refer to assets as `assets::LOGO` (typo-proof, autocompletes)
/// rather than raw strings. Add a file to `assets/` and it appears here.
pub mod assets {
    include!(concat!(env!("OUT_DIR"), "/rosace_assets.rs"));
}

use rosace::prelude::*;

/// Start the app. Runs the winit event loop on native; hands off to the
/// browser's requestAnimationFrame loop on web.
pub fn launch() {
    // Window size applies on desktop; mobile is always fullscreen.
    App::new()
        .title("gallery_all")
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
