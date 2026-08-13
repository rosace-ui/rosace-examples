//! Platform Channel (D127) — a live, end-to-end demo of ROSACE's generic
//! bridge to native platform code, in the style of Flutter's own
//! `MethodChannel` tutorial (the same "battery level" / "device info"
//! example most mobile developers have already seen once).
//!
//! **Why this exists**: ROSACE's core (this file, the whole `rosace`
//! crate) never needs to know about `UIDevice`, `AVCaptureDevice`,
//! `android.os.Build`, or any other platform-specific API. Instead, this
//! screen asks NATIVE code (Swift on iOS, Kotlin on Android — see
//! `ios/App/EngineViewController.swift` and
//! `android/.../MainActivity.kt`) to do that platform-specific work, and
//! gets an answer back through a small, generic JSON bridge. This is how
//! you'd wire up ANY native SDK or OS API ROSACE doesn't have first-class
//! support for.
//!
//! Two independent call directions, both demonstrated below:
//!
//! 1. **Rust calls native, asynchronously** (`rosace_ffi::invoke_method`) —
//!    the common case. Native might take a while to answer (a system
//!    permission dialog can take as long as the user does), so the call is
//!    queued and answered back later. See "Device Info" and "Camera
//!    Permission" below.
//! 2. **Native calls Rust, synchronously** (registered via
//!    `rosace_ffi::set_method_call_handler` in `lib.rs`) — for when NATIVE
//!    code (a home-screen widget, a Siri Shortcut, a notification action —
//!    anything outside ROSACE's own UI) needs an answer from your Rust
//!    logic immediately. See "Sync Dispatch Self-Test" below; both
//!    `EngineViewController.swift`/`MainActivity.kt` call into this
//!    direction once at launch, logged to the native console, so you can
//!    see the real round trip happen even without a native UI element to
//!    trigger it from.
//!
//! **Platform note**: Platform Channel's native side only exists on iOS
//! and Android right now (see `ios/App/EngineViewController.swift` /
//! `android/.../MainActivity.kt`) — nothing polls the outgoing-call queue
//! on desktop yet, so on macOS/Windows/Linux the async calls below will
//! sit in "Pending" forever. That's expected, not a bug in this demo.

use rosace::prelude::*;
use rosace_ffi::ChannelCallState;
use serde_json::Value;

use crate::MATH_CHANNEL;

/// Renders a `ChannelCallState` as a short status line + result/error text
/// — shared by every card below so the three demos read consistently.
fn status_widget(state: &Option<ChannelCallState>) -> BoxedWidget {
    match state {
        None => Box::new(Text::new("Not asked yet.").color(Color::rgb(140, 140, 140))),
        Some(ChannelCallState::Pending) => {
            Box::new(Text::new("Asking native… (waiting for a reply)").color(Color::rgb(200, 140, 0)))
        }
        Some(ChannelCallState::Resolved(value)) => {
            Box::new(Text::new(format!("✅ {value}")).color(Color::rgb(30, 140, 60)))
        }
        Some(ChannelCallState::Failed(message)) => {
            Box::new(Text::new(format!("❌ {message}")).color(Color::rgb(180, 40, 40)))
        }
    }
}

/// **Demo 1 — Rust calls native, asynchronously, for something instant.**
///
/// "What OS/version is this?" always answers immediately in practice, but
/// the call still goes through the async path — that's the RIGHT choice
/// whenever native *could* take a moment, even if today it usually
/// doesn't. `invoke_method` returns a reactive `Atom` the instant you call
/// it (holding `Pending`); reading `.get()` inside `build()` (as
/// `status_widget` does) subscribes this component to it, so the UI
/// updates on its own the moment native reports back — no polling loop of
/// your own required.
fn device_info_card(call: &Atom<Option<Atom<ChannelCallState>>>) -> BoxedWidget {
    let current = call.get().as_ref().map(|atom| atom.get());
    let call_for_press = call.clone();
    Box::new(
        Card::new(
            Column::new()
                .spacing(8.0)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .child(Text::title("1. Device Info"))
                .child(Text::new(
                    "Rust asks native \u{2192} answered on the next frame \u{2192} \
                     the UI updates reactively, no polling needed.",
                ))
                .child(Button::new("Get OS Version").on_press(move || {
                    let result_atom = rosace_ffi::invoke_method(
                        "dev.rosace.showcase/device",
                        "getOsVersion",
                        Value::Null,
                    );
                    // set_always, not set: Atom<ChannelCallState> can't
                    // implement PartialEq (it's a handle, not a value), so
                    // the normal equal-write dedup (Atom::set) isn't
                    // available here — the framework's documented escape
                    // hatch for exactly this case.
                    call_for_press.set_always(Some(result_atom));
                }))
                .child(status_widget(&current)),
        )
        .padding(EdgeInsets::all(16.0)),
    )
}

/// **Demo 2 — Rust calls native, asynchronously, for something that
/// genuinely takes a while.**
///
/// This is `rosace_ffi::request_camera()` — a real, already-built-in ROSACE
/// capability (not something this demo app invented), which is itself
/// implemented on top of the exact same `invoke_method` mechanism Demo 1
/// uses (see `rosace-ffi::capability`'s source — "a second capability
/// would follow this exact same shape, not a new architecture", now true
/// for a third: your own custom channels). The difference from Demo 1:
/// native's answer here depends on the USER (how long the permission
/// dialog stays up), which is exactly the case async calls exist for.
///
/// **A subtlety worth knowing**: `CAMERA_PERMISSION` (and `PUSH_PERMISSION`)
/// are `GlobalAtom`s, not `ctx.state()` atoms — reading one with a bare
/// `.get()` inside a widget does NOT subscribe that widget to changes
/// (`GlobalAtom`s aren't auto-subscribed the way `ctx.state` hooks are).
/// The correct way is `rosace_ffi::use_camera_permission(ctx)`, called once
/// in `AppRoot::build` (see `app.rs`) and threaded down as a plain value —
/// same reason `home_screen`/`widget_list_screen` take already-created
/// state instead of reading globals themselves.
fn camera_permission_card(permission: Option<bool>) -> BoxedWidget {
    let status: BoxedWidget = match permission {
        None => Box::new(Text::new("Not asked yet.").color(Color::rgb(140, 140, 140))),
        Some(true) => Box::new(Text::new("✅ Granted").color(Color::rgb(30, 140, 60))),
        Some(false) => Box::new(Text::new("❌ Denied").color(Color::rgb(180, 40, 40))),
    };
    Box::new(
        Card::new(
            Column::new()
                .spacing(8.0)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .child(Text::title("2. Camera Permission"))
                .child(Text::new(
                    "A REAL OS permission prompt — this can take as long as \
                     the user takes to tap Allow/Don't Allow. Built into \
                     ROSACE already (rosace_ffi::request_camera); this app's \
                     native code is what actually shows the native \
                     AVCaptureDevice/CAMERA prompt (see EngineViewController.\
                     swift / MainActivity.kt).",
                ))
                .child(Button::new("Request Camera Permission").on_press(|| {
                    rosace_ffi::request_camera();
                }))
                .child(status),
        )
        .padding(EdgeInsets::all(16.0)),
    )
}

/// **Demo 3 — Native calls Rust, synchronously.**
///
/// The reverse direction from Demos 1-2: here NATIVE is the caller. This
/// screen doesn't (and can't cleanly) trigger that side of the round trip
/// itself — a real use of this direction is native code OUTSIDE ROSACE's
/// own UI (a home-screen widget, a Siri Shortcut, a notification action)
/// needing an answer from your Rust logic. To prove the mechanism really
/// works end-to-end, `EngineViewController.swift` and `MainActivity.kt`
/// both call `rsc_platform_channel_dispatch("...math", "add", "[2,3]")`
/// once at launch and log the result to the native console (Xcode's
/// console / `adb logcat`) — open either while running this app and you'll
/// see "Platform Channel self-test: 2 + 3 = 5" (or similar) printed by
/// NATIVE code that called INTO this Rust handler (registered in
/// `lib.rs::app_init`) and got a real answer back, synchronously, in one
/// blocking call.
fn sync_dispatch_card() -> BoxedWidget {
    Box::new(
        Card::new(
            Column::new()
                .spacing(8.0)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .child(Text::title("3. Sync Dispatch Self-Test"))
                .child(Text::new(format!(
                    "Native calls a Rust handler registered on \"{MATH_CHANNEL}\" \
                     and gets an answer back in ONE blocking call — no queue, \
                     no atom, just a normal function call across the FFI \
                     boundary. Check the native console (Xcode / adb logcat) \
                     for the self-test this app's native code runs at launch."
                )))
                .child(Text::new("(there's nothing to tap here — see the doc comment above)")
                    .color(Color::rgb(140, 140, 140))),
        )
        .padding(EdgeInsets::all(16.0)),
    )
}

pub fn platform_channel_screen(
    device_info_call: &Atom<Option<Atom<ChannelCallState>>>,
    camera_permission: Option<bool>,
) -> impl Widget {
    ScrollView::new(
        Column::new()
            .spacing(12.0)
            .padding(EdgeInsets::all(16.0))
            .child(device_info_card(device_info_call))
            .child(camera_permission_card(camera_permission))
            .child(sync_dispatch_card()),
    )
}
