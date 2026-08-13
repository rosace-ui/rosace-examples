//! Native-host FFI glue (D106 Phase 24) — exports the ABI
//! `ios/App/EngineViewController.swift` and `android/.../MainActivity.kt`
//! call into. iOS uses the plain C ABI in `rosace-ffi`'s
//! `include/rsc_engine.h` (pattern: `rosace-ffi/examples/ios_stub.rs`).
//! Android uses JNI instead — Kotlin's `external fun` resolves to a symbol
//! literally named `Java_<package>_<Class>_<method>` (JNI's mangling: `.` ->
//! `_`, a literal `_` -> `_1` — see `jni_class_prefix` in
//! `rosace-cli/src/commands/new.rs`, which computed the exact prefix below
//! from this app's bundle id at `rsc new` time). Pattern:
//! `rosace-ffi/examples/android_stub.rs`.

use std::os::raw::c_void;
#[cfg(target_os = "ios")]
use std::ptr::NonNull;

#[cfg(any(target_os = "ios", target_os = "android"))]
use rosace::prelude::*;
use rosace_ffi::{Engine, RscInputEventFfi};
#[cfg(target_os = "ios")]
use rosace_ffi::RawSurface;
#[cfg(target_os = "android")]
use rosace_ffi::AndroidSurfaceHandle;

#[cfg(any(target_os = "ios", target_os = "android"))]
use crate::app::AppRoot;

// -- iOS: plain C ABI --------------------------------------------------------

/// # Safety
/// `surface_handle` must be a valid, non-null `CAMetalLayer`-backed
/// `UIView*` for the engine's lifetime.
#[cfg(target_os = "ios")]
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_init(
    surface_handle: *mut c_void,
    width: u32,
    height: u32,
    scale: f32,
) -> *mut Engine {
    let Some(handle) = NonNull::new(surface_handle) else { return std::ptr::null_mut() };
    let surface = unsafe { RawSurface::from_ca_metal_layer(handle, None, width, height, scale) };
    let theme = light_theme();
    // Mobile bypasses lib.rs's launch() entirely — app_init() must be
    // called explicitly here too, or one-time app setup silently never
    // runs on iOS (see app_init's doc in lib.rs for why).
    crate::app_init();
    match Engine::init(Box::new(AppRoot), theme, surface) {
        Some(engine) => Box::into_raw(engine),
        None => std::ptr::null_mut(),
    }
}

#[cfg(not(target_os = "ios"))]
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_init(
    _surface_handle: *mut c_void,
    _width: u32,
    _height: u32,
    _scale: f32,
) -> *mut Engine {
    std::ptr::null_mut()
}

/// # Safety
/// `engine` must be a live pointer previously returned by `rsc_engine_init`
/// (or null, which is a no-op).
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_resize(
    engine: *mut Engine,
    width: u32,
    height: u32,
    scale: f32,
    safe_top: f32,
    safe_right: f32,
    safe_bottom: f32,
    safe_left: f32,
) {
    if engine.is_null() { return; }
    let safe_area = rosace::core::SafeArea { top: safe_top, right: safe_right, bottom: safe_bottom, left: safe_left };
    unsafe { (*engine).resize(width, height, scale, safe_area) };
}

/// # Safety
/// `engine` must be a live pointer previously returned by `rsc_engine_init`
/// (or null, which is a no-op). Called by the native host whenever the OS
/// reports an appearance/accessibility change (iOS
/// `traitCollectionDidChange`, desktop `WindowEvent::ThemeChanged`, web
/// `matchMedia` `"change"` — see each platform's native glue for exactly
/// which fields it can source).
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_set_media_query(
    engine: *mut Engine,
    is_dark: u8,
    text_scale: f32,
    bold_text: u8,
    reduce_motion: u8,
    always_24_hour_format: u8,
) {
    if engine.is_null() { return; }
    let mq = rosace::core::MediaQuery {
        text_scale,
        is_dark: is_dark != 0,
        bold_text: bold_text != 0,
        reduce_motion: reduce_motion != 0,
        always_24_hour_format: always_24_hour_format != 0,
    };
    unsafe { (*engine).set_media_query(mq) };
}

/// # Safety
/// `engine` must be a live pointer from `rsc_engine_init`; `events` must
/// point to at least `count` valid `RscInputEvent`s.
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_input(
    engine: *mut Engine,
    events: *const RscInputEventFfi,
    count: usize,
) {
    if engine.is_null() || events.is_null() { return; }
    let slice = unsafe { std::slice::from_raw_parts(events, count) };
    unsafe { (*engine).input(slice) };
}

/// # Safety
/// `engine` must be a live pointer from `rsc_engine_init` (or null).
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_frame(engine: *mut Engine) {
    if engine.is_null() { return; }
    unsafe { (*engine).frame() };
}

/// # Safety
/// `engine` must be a pointer previously returned by `rsc_engine_init` and
/// not yet passed to this function; it must not be used again afterward.
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_shutdown(engine: *mut Engine) {
    if engine.is_null() { return; }
    drop(unsafe { Box::from_raw(engine) });
}

// -- Push notifications (D110 Phase 29 Step 2) --------------------------------
// Discovery ("is a permission request pending?") goes through the generic
// Platform Channel poll below, not a dedicated take_request — see D127 and
// rosace_ffi::capability's module doc. Result-reporting stays a plain
// setter (no call_id correlation needed for a singleton capability).

#[no_mangle]
pub extern "C" fn rsc_push_permission_report_result(granted: u8) {
    rosace_ffi::report_push_result(granted != 0);
}

// -- Camera permission (Platform Channel demo) --------------------------------
// Camera is NOT part of the shared rsc-cli generator template (wiring it
// unconditionally into every app would bake an unused NSCameraUsageDescription
// into Info.plist for apps that never touch the camera) — this export is
// app-specific, added by hand for this demo, exactly the "an app adds a case
// for its own channel" story the generator's pollPlatformChannel doc
// describes. Discovery goes through the generic poll (see
// ios/App/EngineViewController.swift's pollPlatformChannel); result-reporting
// is a plain setter, same shape as push above.

#[no_mangle]
pub extern "C" fn rsc_camera_permission_report_result(granted: u8) {
    rosace_ffi::report_camera_result(granted != 0);
}

/// # Safety
/// `token` must be a valid NUL-terminated C string or null (a no-op).
#[no_mangle]
pub unsafe extern "C" fn rsc_push_report_token(token: *const std::os::raw::c_char) {
    if token.is_null() { return; }
    let token = unsafe { std::ffi::CStr::from_ptr(token) }.to_string_lossy().into_owned();
    rosace_ffi::report_push_token(token);
}

/// # Safety
/// Each argument must be a valid NUL-terminated C string or null (null
/// reads as the empty string; the call still delivers).
#[no_mangle]
pub unsafe extern "C" fn rsc_push_report_notification(
    title: *const std::os::raw::c_char,
    body: *const std::os::raw::c_char,
    payload_json: *const std::os::raw::c_char,
) {
    let read = |p: *const std::os::raw::c_char| -> String {
        if p.is_null() {
            String::new()
        } else {
            unsafe { std::ffi::CStr::from_ptr(p) }.to_string_lossy().into_owned()
        }
    };
    rosace_ffi::report_push_notification(read(title), read(body), read(payload_json));
}

// -- Platform Channel (D127) ---------------------------------------------------
// The generic bidirectional method-call bridge to native code — named
// channels + methods + JSON payloads, instead of a bespoke FFI function per
// platform feature. Two directions, four exports:
//   - Rust calls native, async: `rsc_platform_channel_take_outgoing` (the
//     host's ONE per-frame poll — this is what push permission discovery
//     above now goes through, alongside anything an app registers itself)
//     + `rsc_platform_channel_report_result`/`_report_error` (the host
//     answers once its native-side work finishes, which may be many frames
//     later — a system dialog, a slow SDK call).
//   - Native calls Rust, sync: `rsc_platform_channel_dispatch` — one
//     blocking call, answered inline by whatever handler the app registered
//     via `rosace_ffi::set_method_call_handler`. For fast work only.
// `rsc_string_free` pairs with every owned string this crate returns across
// the boundary (`take_outgoing`'s JSON array, `dispatch`'s JSON result) —
// the receiver must call it exactly once after copying the bytes into its
// own native string, same discipline `AndroidSurfaceHandle`'s `Drop`
// already follows for the native-window reference.

/// # Safety
/// The returned pointer is an owned, NUL-terminated JSON string (a `[]`
/// array of `{call_id, channel, method, args}` objects) that the caller
/// MUST pass to `rsc_string_free` exactly once when done reading it.
#[no_mangle]
pub extern "C" fn rsc_platform_channel_take_outgoing() -> *mut std::os::raw::c_char {
    let calls: Vec<serde_json::Value> = rosace_ffi::take_outgoing_calls()
        .into_iter()
        .map(|c| {
            serde_json::json!({
                "call_id": c.call_id,
                "channel": c.channel,
                "method": c.method,
                "args": serde_json::from_str::<serde_json::Value>(&c.args_json)
                    .unwrap_or(serde_json::Value::Null),
            })
        })
        .collect();
    let text = serde_json::Value::Array(calls).to_string();
    std::ffi::CString::new(text).unwrap_or_default().into_raw()
}

/// The current accessibility tree as JSON (D132) — what the native host
/// republishes to VoiceOver (iOS) / TalkBack (Android).
///
/// PULL, not push: both mobile a11y APIs are demand-driven, so the host
/// calls this only while assistive tech is actually inspecting. An app with
/// no screen reader running never pays for it.
///
/// `bounds` are LOGICAL pixels, window-relative — each host converts to its
/// own convention (iOS screen-space `CGRect`, Android physical-pixel `Rect`).
///
/// # Safety
/// `engine` must be a live pointer from `rsc_engine_init`. The returned
/// pointer is an owned, NUL-terminated JSON string the caller MUST pass to
/// `rsc_string_free` exactly once.
#[no_mangle]
pub unsafe extern "C" fn rsc_engine_semantics_json(
    engine: *mut Engine,
) -> *mut std::os::raw::c_char {
    if engine.is_null() {
        return std::ptr::null_mut();
    }
    let engine = unsafe { &*engine };
    let text = rosace_ffi::semantics_json(engine);
    std::ffi::CString::new(text).unwrap_or_default().into_raw()
}

/// Frees a string previously returned by `rsc_platform_channel_take_outgoing`,
/// `rsc_platform_channel_dispatch`, or `rsc_engine_semantics_json`.
///
/// # Safety
/// `ptr` must be either null (a no-op) or a pointer this crate returned
/// across the FFI boundary, not yet freed.
#[no_mangle]
pub unsafe extern "C" fn rsc_string_free(ptr: *mut std::os::raw::c_char) {
    if ptr.is_null() { return; }
    drop(unsafe { std::ffi::CString::from_raw(ptr) });
}

/// # Safety
/// `result_json` must be a valid NUL-terminated C string or null (a no-op).
#[no_mangle]
pub unsafe extern "C" fn rsc_platform_channel_report_result(
    call_id: u64,
    result_json: *const std::os::raw::c_char,
) {
    if result_json.is_null() { return; }
    let json = unsafe { std::ffi::CStr::from_ptr(result_json) }.to_string_lossy();
    rosace_ffi::report_call_result(call_id, &json);
}

/// # Safety
/// `message` must be a valid NUL-terminated C string or null (a no-op).
#[no_mangle]
pub unsafe extern "C" fn rsc_platform_channel_report_error(
    call_id: u64,
    message: *const std::os::raw::c_char,
) {
    if message.is_null() { return; }
    let msg = unsafe { std::ffi::CStr::from_ptr(message) }.to_string_lossy().into_owned();
    rosace_ffi::report_call_error(call_id, msg);
}

/// # Safety
/// Each argument must be a valid NUL-terminated C string or null (null
/// reads as the empty string). The returned pointer is owned — see the
/// module doc's note on `rsc_string_free`.
#[no_mangle]
pub unsafe extern "C" fn rsc_platform_channel_dispatch(
    channel: *const std::os::raw::c_char,
    method: *const std::os::raw::c_char,
    args_json: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let read = |p: *const std::os::raw::c_char| -> String {
        if p.is_null() {
            String::new()
        } else {
            unsafe { std::ffi::CStr::from_ptr(p) }.to_string_lossy().into_owned()
        }
    };
    let result = rosace_ffi::dispatch_call(&read(channel), &read(method), &read(args_json));
    std::ffi::CString::new(result).unwrap_or_default().into_raw()
}

// -- Soft-keyboard sync (D116 Step 6) -----------------------------------------
// Shared, platform-agnostic (like the push functions above): a native host
// polls these once per frame tick to know whether to show/hide its OS soft
// keyboard and which layout to use — iOS via `@_silgen_name`, Android through
// the JNI wrappers below. No engine handle needed; these read the same
// process-global focus signal `ime_cursor_area`/`keyboard_type` already use
// for desktop's real OS IME.

#[no_mangle]
pub extern "C" fn rsc_text_input_active() -> u8 {
    rosace_ffi::text_input_active() as u8
}

#[no_mangle]
pub extern "C" fn rsc_focused_keyboard_type() -> u32 {
    rosace_ffi::focused_keyboard_type()
}

// -- Android: JNI -------------------------------------------------------------
// Symbol names are burned in at codegen time (JNI resolves by exact name,
// no runtime registration) — see the module doc above for why this can't be
// the same plain-C functions iOS uses. `AndroidEngine` keeps the `Engine`
// and the `AndroidSurfaceHandle` (whose `Drop` releases the `ANativeWindow`
// reference) alive together, torn down as a unit in nativeShutdown — same
// reasoning as `rosace-ffi/examples/android_stub.rs`'s `AndroidEngine`.

#[cfg(target_os = "android")]
struct AndroidEngine {
    engine: Box<Engine>,
    #[allow(dead_code)]
    surface: AndroidSurfaceHandle,
}

/// NEVER called — winit's android-native-activity backend references this
/// symbol from its NativeActivity glue (rosace-platform compiles winit for
/// android so the shared types typecheck; see its Cargo.toml note), and
/// without a definition the final cdylib carries an undefined symbol that
/// makes `System.loadLibrary` fail with `UnsatisfiedLinkError` at app
/// startup. The D106 host drives the app entirely via the JNI functions
/// above; winit's own Android entry path is deliberately unused.
#[cfg(target_os = "android")]
#[no_mangle]
extern "C" fn android_main(_app: *mut std::ffi::c_void) {
    unreachable!("NativeActivity entry is unused — the JNI host owns the app (D106)");
}

// Android discards a process's stderr, so a Rust panic normally vanishes
// with no trace in `adb logcat` — the app just dies. Route panics to logcat
// (`liblog`) as FATAL so they're visible. Installed once, idempotently, from
// nativeInit before any engine work runs.
#[cfg(target_os = "android")]
#[link(name = "log")]
extern "C" {
    fn __android_log_write(
        prio: std::os::raw::c_int,
        tag: *const std::os::raw::c_char,
        text: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
}

#[cfg(target_os = "android")]
fn install_panic_logcat() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            let tag = std::ffi::CString::new("rosace").unwrap();
            let text = std::ffi::CString::new(format!("{info}"))
                .unwrap_or_else(|_| std::ffi::CString::new("panic (unprintable message)").unwrap());
            // ANDROID_LOG_FATAL = 7
            unsafe { __android_log_write(7, tag.as_ptr(), text.as_ptr()); }
        }));
    });
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeInit(
    env: jni::JNIEnv,
    _class: jni::objects::JObject,
    surface: jni::objects::JObject,
    width: jni::sys::jint,
    height: jni::sys::jint,
    scale: jni::sys::jfloat,
) -> jni::sys::jlong {
    install_panic_logcat();
    let raw_env = env.get_raw();
    let Some(handle) = (unsafe { AndroidSurfaceHandle::from_jni(raw_env, &surface) }) else {
        return 0;
    };
    let raw_surface = unsafe { handle.raw_surface(width as u32, height as u32, scale) };
    let theme = light_theme();
    // Mobile bypasses lib.rs's launch() entirely — app_init() must be
    // called explicitly here too, or one-time app setup silently never
    // runs on Android (see app_init's doc in lib.rs for why).
    crate::app_init();
    match Engine::init(Box::new(AppRoot), theme, raw_surface) {
        Some(engine) => Box::into_raw(Box::new(AndroidEngine { engine, surface: handle })) as jni::sys::jlong,
        None => 0,
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeResize(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
    width: jni::sys::jint,
    height: jni::sys::jint,
    scale: jni::sys::jfloat,
    safe_top: jni::sys::jfloat,
    safe_right: jni::sys::jfloat,
    safe_bottom: jni::sys::jfloat,
    safe_left: jni::sys::jfloat,
) {
    if handle == 0 { return; }
    let ptr = handle as *mut AndroidEngine;
    let safe_area = rosace::core::SafeArea { top: safe_top, right: safe_right, bottom: safe_bottom, left: safe_left };
    unsafe { (*ptr).engine.resize(width as u32, height as u32, scale, safe_area) };
}

/// Called once from `nativeInit` and again from every
/// `onConfigurationChanged` (uiMode/fontScale changes) — see `MainActivity.kt`.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeSetMediaQuery(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
    is_dark: jni::sys::jboolean,
    text_scale: jni::sys::jfloat,
    bold_text: jni::sys::jboolean,
    reduce_motion: jni::sys::jboolean,
    always_24_hour_format: jni::sys::jboolean,
) {
    if handle == 0 { return; }
    let ptr = handle as *mut AndroidEngine;
    let mq = rosace::core::MediaQuery {
        text_scale,
        is_dark: is_dark != 0,
        bold_text: bold_text != 0,
        reduce_motion: reduce_motion != 0,
        always_24_hour_format: always_24_hour_format != 0,
    };
    unsafe { (*ptr).engine.set_media_query(mq) };
}

/// One touch/pointer event per call — `kind` is `0` = move, `1` = down,
/// `2` = up (matching `rosace_ffi`'s `RSC_EVENT_MOUSE_*` constants); a
/// touch is always reported as the left button, mirroring how the existing
/// winit `Touch` handling already treats touch input (see `rosace-ffi`'s
/// `event.rs` module doc).
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeTouch(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
    kind: jni::sys::jint,
    x: jni::sys::jfloat,
    y: jni::sys::jfloat,
) {
    if handle == 0 { return; }
    let ptr = handle as *mut AndroidEngine;
    let event = RscInputEventFfi {
        kind: kind as u32, x, y, button: 0, key: 0, character: 0,
        width: 0, height: 0, delta_x: 0.0, delta_y: 0.0,
    };
    unsafe { (*ptr).engine.input(&[event]) };
}

/// One key event per call — `key` is an `RSC_KEY_*` constant (matching
/// `rosace_ffi::event`'s desktop/iOS key encoding); `kind` 3 = KeyDown (see
/// `rosace_ffi::event::RSC_EVENT_KEY_DOWN` — not re-exported, so burned in as
/// a literal here, same as `nativeLifecycle`'s kinds below). Used for
/// Backspace, Enter, and Tab, which the engine's editor treats as commands
/// rather than literal text (see `nativeText` below for typed characters).
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeKey(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
    key: jni::sys::jint,
) {
    if handle == 0 { return; }
    let ptr = handle as *mut AndroidEngine;
    let event = RscInputEventFfi {
        kind: 3, x: 0.0, y: 0.0, button: 0,
        key: key as u32, character: 0, width: 0, height: 0, delta_x: 0.0, delta_y: 0.0,
    };
    unsafe { (*ptr).engine.input(&[event]) };
}

/// One typed Unicode scalar per call — `kind` 5 = Text (`RSC_EVENT_TEXT`).
/// The IME's `commitText` forwards each character here (mirroring iOS's
/// `UIKeyInput.insertText`); Enter/Tab are sent through `nativeKey` instead,
/// never as text (see the Kotlin `InputConnection` this backs for why).
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeText(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
    character: jni::sys::jint,
) {
    if handle == 0 { return; }
    let ptr = handle as *mut AndroidEngine;
    let event = RscInputEventFfi {
        kind: 5, x: 0.0, y: 0.0, button: 0,
        key: 0, character: character as u32, width: 0, height: 0, delta_x: 0.0, delta_y: 0.0,
    };
    unsafe { (*ptr).engine.input(&[event]) };
}

/// Whether a text field is currently focused (D116 Step 6) — polled once per
/// frame tick (mirroring iOS's `rsc_text_input_active`) to decide whether to
/// show/hide the soft keyboard. No handle needed: same process-global focus
/// signal desktop's real OS IME already uses.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeTextInputActive(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
) -> jni::sys::jboolean {
    rosace_ffi::text_input_active() as jni::sys::jboolean
}

/// The focused field's keyboard-type hint, an `RSC_KEYBOARD_*` constant
/// (mirroring iOS's `rsc_focused_keyboard_type`) — used to pick the IME's
/// `inputType` (email/numeric/URL/phone/default).
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeFocusedKeyboardType(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
) -> jni::sys::jint {
    rosace_ffi::focused_keyboard_type() as jni::sys::jint
}

// -- Platform Channel (D127) — JNI wrappers around the same rosace_ffi
// primitives the iOS plain-C exports above use. JNI strings are JVM-managed
// (`env.new_string`/`get_string`), unlike iOS's C strings — no
// `rsc_string_free` equivalent is needed here; the JVM garbage-collects
// `JString`s normally.

/// The engine's accessibility tree as JSON (D132) — what the Kotlin side
/// turns into `AccessibilityNodeInfo`s for TalkBack.
///
/// PULL, not push: `AccessibilityNodeProvider` is called only while an
/// accessibility service is actually exploring, so an app with TalkBack off
/// never serializes anything. Bounds are LOGICAL, view-relative pixels; the
/// host multiplies by density for `AccessibilityNodeInfo`'s physical-pixel
/// `Rect`.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeSemanticsJson(
    env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
) -> jni::sys::jstring {
    if handle == 0 {
        return std::ptr::null_mut();
    }
    // The handle is an `AndroidEngine` (the surface-owning wrapper), NOT a
    // bare `Engine` — every other JNI fn here casts it that way. Casting to
    // `Engine` directly read from a bogus offset and segfaulted at 0x10 the
    // moment TalkBack/uiautomator first queried the tree.
    let ptr = handle as *mut AndroidEngine;
    let text = rosace_ffi::semantics_json(unsafe { &(*ptr).engine });
    env.new_string(text).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut())
}

/// The host's ONE per-frame poll (alongside `nativeFrame`) — drains every
/// queued Platform Channel call (push-permission discovery included) as a
/// JSON array of `{call_id, channel, method, args}` objects.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeTakeOutgoingPlatformCalls(
    env: jni::JNIEnv,
    _class: jni::objects::JObject,
) -> jni::sys::jstring {
    let calls: Vec<serde_json::Value> = rosace_ffi::take_outgoing_calls()
        .into_iter()
        .map(|c| {
            serde_json::json!({
                "call_id": c.call_id,
                "channel": c.channel,
                "method": c.method,
                "args": serde_json::from_str::<serde_json::Value>(&c.args_json)
                    .unwrap_or(serde_json::Value::Null),
            })
        })
        .collect();
    let text = serde_json::Value::Array(calls).to_string();
    env.new_string(text).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut())
}

/// Called once `call_id`'s native-side work finishes successfully.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativePlatformChannelReportResult(
    mut env: jni::JNIEnv,
    _class: jni::objects::JObject,
    call_id: jni::sys::jlong,
    result_json: jni::objects::JString,
) {
    let json: String = env.get_string(&result_json).map(String::from).unwrap_or_default();
    rosace_ffi::report_call_result(call_id as u64, &json);
}

/// Called when `call_id`'s native-side work fails.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativePlatformChannelReportError(
    mut env: jni::JNIEnv,
    _class: jni::objects::JObject,
    call_id: jni::sys::jlong,
    message: jni::objects::JString,
) {
    let msg: String = env.get_string(&message).map(String::from).unwrap_or_default();
    rosace_ffi::report_call_error(call_id as u64, msg);
}

/// Native calls INTO Rust, synchronously — one blocking call answered
/// inline by whatever handler the app registered via
/// `rosace_ffi::set_method_call_handler`. For fast work only.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativePlatformChannelDispatch(
    mut env: jni::JNIEnv,
    _class: jni::objects::JObject,
    channel: jni::objects::JString,
    method: jni::objects::JString,
    args_json: jni::objects::JString,
) -> jni::sys::jstring {
    let channel: String = env.get_string(&channel).map(String::from).unwrap_or_default();
    let method: String = env.get_string(&method).map(String::from).unwrap_or_default();
    let args: String = env.get_string(&args_json).map(String::from).unwrap_or_default();
    let result = rosace_ffi::dispatch_call(&channel, &method, &args);
    env.new_string(result).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut())
}

/// Camera is app-specific, not part of the shared generator — mirrors
/// `rsc_camera_permission_report_result` in the iOS half of this same file.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeCameraPermissionReportResult(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    granted: jni::sys::jint,
) {
    rosace_ffi::report_camera_result(granted != 0);
}

/// One app-lifecycle transition per call (D110 Phase 29 Step 1) — `kind`
/// is a `RSC_EVENT_LIFECYCLE_*` constant (8 = active, 9 = inactive,
/// 10 = background). `Engine::input` applies lifecycle immediately (see
/// its doc), so calling this from `onStop` — after the Choreographer
/// callback has gone quiet — still takes effect right away.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeLifecycle(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
    kind: jni::sys::jint,
) {
    if handle == 0 { return; }
    let ptr = handle as *mut AndroidEngine;
    let event = RscInputEventFfi {
        kind: kind as u32, x: 0.0, y: 0.0, button: 0, key: 0, character: 0,
        width: 0, height: 0, delta_x: 0.0, delta_y: 0.0,
    };
    unsafe { (*ptr).engine.input(&[event]) };
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeFrame(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
) {
    if handle == 0 { return; }
    let ptr = handle as *mut AndroidEngine;
    unsafe { (*ptr).engine.frame() };
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_showcase_MainActivity_nativeShutdown(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
) {
    if handle == 0 { return; }
    drop(unsafe { Box::from_raw(handle as *mut AndroidEngine) });
}
