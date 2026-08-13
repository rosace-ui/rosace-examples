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

// -- Soft keyboard / text input (D116 Step 6) ---------------------------------
// Engine-independent global signals the host polls once per frame: whether a
// text field is focused (show/hide the OS keyboard) and which keyboard layout
// to show. Keystrokes flow back the other way, through rsc_engine_input as
// RSC_KEY_CHAR / RSC_KEY_* / RSC_EVENT_KEY_* events.

#[no_mangle]
pub extern "C" fn rsc_text_input_active() -> u8 {
    rosace_ffi::text_input_active() as u8
}

#[no_mangle]
pub extern "C" fn rsc_focused_keyboard_type() -> u32 {
    rosace_ffi::focused_keyboard_type()
}

// -- Push notifications (D110 Phase 29 Step 2) --------------------------------
// Engine-independent, same as the camera capability (see rsc_engine.h's doc)
// — the host polls take_request once per frame tick and reports back.

#[no_mangle]
pub extern "C" fn rsc_push_permission_take_request() -> u8 {
    rosace_ffi::take_push_request() as u8
}

#[no_mangle]
pub extern "C" fn rsc_push_permission_report_result(granted: u8) {
    rosace_ffi::report_push_result(granted != 0);
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
pub extern "system" fn Java_dev_rosace_galleryall_MainActivity_nativeInit(
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
    match Engine::init(Box::new(AppRoot), theme, raw_surface) {
        Some(engine) => Box::into_raw(Box::new(AndroidEngine { engine, surface: handle })) as jni::sys::jlong,
        None => 0,
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_galleryall_MainActivity_nativeResize(
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

/// One touch/pointer event per call — `kind` is `0` = move, `1` = down,
/// `2` = up (matching `rosace_ffi`'s `RSC_EVENT_MOUSE_*` constants); a
/// touch is always reported as the left button, mirroring how the existing
/// winit `Touch` handling already treats touch input (see `rosace-ffi`'s
/// `event.rs` module doc).
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_galleryall_MainActivity_nativeTouch(
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

/// One app-lifecycle transition per call (D110 Phase 29 Step 1) — `kind`
/// is a `RSC_EVENT_LIFECYCLE_*` constant (8 = active, 9 = inactive,
/// 10 = background). `Engine::input` applies lifecycle immediately (see
/// its doc), so calling this from `onStop` — after the Choreographer
/// callback has gone quiet — still takes effect right away.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_dev_rosace_galleryall_MainActivity_nativeLifecycle(
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
pub extern "system" fn Java_dev_rosace_galleryall_MainActivity_nativeFrame(
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
pub extern "system" fn Java_dev_rosace_galleryall_MainActivity_nativeShutdown(
    _env: jni::JNIEnv,
    _class: jni::objects::JObject,
    handle: jni::sys::jlong,
) {
    if handle == 0 { return; }
    drop(unsafe { Box::from_raw(handle as *mut AndroidEngine) });
}
