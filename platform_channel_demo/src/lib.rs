//! platform_channel_demo — a ROSACE app.
//!
//! `launch()` is shared by every platform. The native binary calls it from
//! `main`; the web build calls it from a `wasm-bindgen(start)` entry.

mod app;
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

/// The channel name for this demo's own custom method (the "native calls
/// Rust, synchronously" direction — see `screens::platform_channel`'s doc
/// for the full picture). Channel names are just strings you pick — using
/// your bundle id as a prefix (like a Java package) avoids collisions with
/// other libraries' channels in the same app.
pub const MATH_CHANNEL: &str = "dev.rosace.platformchanneldemo/math";

/// One-time app startup — registers this app's Platform Channel method
/// handler (the "native calls Rust" direction:
/// `rosace_ffi::dispatch_call`/`set_method_call_handler`, wired to each
/// platform's `rsc_platform_channel_dispatch` FFI export).
///
/// Called from EVERY entry point below (`launch`, and — on iOS/Android —
/// `ffi.rs`'s `rsc_engine_init`/`nativeInit`), not just `launch`: mobile's
/// FFI entry points construct the engine directly and never call `launch`,
/// so a handler registered only here would silently never exist on
/// iOS/Android (found live, running this exact demo on a real iOS
/// Simulator: the sync-dispatch self-test answered "no handler registered"
/// until this was called from `ffi.rs` too — see `rsc-cli`'s D127 fix,
/// which made this the standard `app_init()` convention every generated
/// app now follows).
///
/// The "Rust calls native" direction (`rosace_ffi::invoke_method`) needs NO
/// registration — see `screens::platform_channel` for those call sites.
pub(crate) fn app_init() {
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
        .title("platform_channel_demo")
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
