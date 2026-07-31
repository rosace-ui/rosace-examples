//! Drives the ROSACE engine through the `rosace-ffi` C boundary
//! (`rosace-ffi/include/rsc_engine.h`) — a CAMetalLayer-backed view,
//! init/resize/input/frame calls, and real `UIView.safeAreaInsets` feeding
//! `rosace_core::SafeArea` (replacing the old winit outer/inner-size
//! workaround from Phase 20-22).

import UIKit
import QuartzCore
import UserNotifications
import AVFoundation

// MARK: - FFI declarations (mirrors rosace-ffi/include/rsc_engine.h;
// no bridging header needed — matches the pattern proven in
// rosace-ffi/examples/ios_stub.rs's Simulator verification).

typealias RscEngine = OpaquePointer

struct RscInputEvent {
    var kind: UInt32
    var x: Float
    var y: Float
    var button: UInt32
    var key: UInt32
    var character: UInt32
    var width: UInt32
    var height: UInt32
    var delta_x: Float
    var delta_y: Float
}

private let RSC_EVENT_MOUSE_MOVE: UInt32 = 0
private let RSC_EVENT_MOUSE_DOWN: UInt32 = 1
private let RSC_EVENT_MOUSE_UP: UInt32 = 2
private let RSC_BUTTON_LEFT: UInt32 = 0
// Text input (D116 Step 6): typed characters go as RSC_EVENT_TEXT; backspace
// as a RSC_EVENT_KEY_DOWN carrying RSC_KEY_BACKSPACE — same events the desktop
// winit host produces, so the engine's editor handles them identically.
private let RSC_EVENT_KEY_DOWN: UInt32 = 3
private let RSC_EVENT_TEXT: UInt32 = 5
private let RSC_KEY_ENTER: UInt32 = 0
private let RSC_KEY_BACKSPACE: UInt32 = 3
private let RSC_KEY_TAB: UInt32 = 4
private let RSC_EVENT_LIFECYCLE_ACTIVE: UInt32 = 8
private let RSC_EVENT_LIFECYCLE_INACTIVE: UInt32 = 9
private let RSC_EVENT_LIFECYCLE_BACKGROUND: UInt32 = 10
private let RSC_EVENT_LIFECYCLE_SUSPENDED: UInt32 = 11

@_silgen_name("rsc_engine_init")
func rsc_engine_init(_ surfaceHandle: UnsafeMutableRawPointer?, _ width: UInt32, _ height: UInt32, _ scale: Float) -> RscEngine?

@_silgen_name("rsc_engine_resize")
func rsc_engine_resize(
    _ engine: RscEngine?, _ width: UInt32, _ height: UInt32, _ scale: Float,
    _ safeTop: Float, _ safeRight: Float, _ safeBottom: Float, _ safeLeft: Float
)

@_silgen_name("rsc_engine_input")
func rsc_engine_input(_ engine: RscEngine?, _ events: UnsafePointer<RscInputEvent>?, _ count: Int)

@_silgen_name("rsc_engine_frame")
func rsc_engine_frame(_ engine: RscEngine?)

@_silgen_name("rsc_engine_shutdown")
func rsc_engine_shutdown(_ engine: RscEngine?)

@_silgen_name("rsc_push_permission_report_result")
func rsc_push_permission_report_result(_ granted: UInt8)

// Camera is app-specific, not part of the shared generator (see ffi.rs's
// doc on rsc_camera_permission_report_result for why).
@_silgen_name("rsc_camera_permission_report_result")
func rsc_camera_permission_report_result(_ granted: UInt8)

@_silgen_name("rsc_text_input_active")
func rsc_text_input_active() -> UInt8

@_silgen_name("rsc_focused_keyboard_type")
func rsc_focused_keyboard_type() -> UInt32

// MARK: - Platform Channel (D127) — the generic bidirectional method-call
// bridge to native code. `take_outgoing`/`report_result`/`report_error`
// replace the old dedicated push-permission-only poll — that discovery now
// goes through this same generic queue, alongside anything an app registers
// itself. `dispatch` is the reverse direction (native calling a
// Rust-registered handler), included for completeness
// even though this template doesn't call it itself.

@_silgen_name("rsc_platform_channel_take_outgoing")
func rsc_platform_channel_take_outgoing() -> UnsafeMutablePointer<CChar>?

@_silgen_name("rsc_string_free")
func rsc_string_free(_ ptr: UnsafeMutablePointer<CChar>?)

@_silgen_name("rsc_platform_channel_report_result")
func rsc_platform_channel_report_result(_ callId: UInt64, _ resultJson: UnsafePointer<CChar>?)

@_silgen_name("rsc_platform_channel_report_error")
func rsc_platform_channel_report_error(_ callId: UInt64, _ message: UnsafePointer<CChar>?)

@_silgen_name("rsc_platform_channel_dispatch")
func rsc_platform_channel_dispatch(
    _ channel: UnsafePointer<CChar>?, _ method: UnsafePointer<CChar>?, _ argsJson: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>?

// MARK: - View

/// A `CAMetalLayer`-backed view — the surface the Rust engine renders into.
///
/// `contentsScale` is set explicitly in `init` — UIKit only auto-syncs a
/// view's OWN default `CALayer` to the screen's pixel density; overriding
/// `layerClass` with a custom layer (as this does) opts out of that
/// automatic behavior, and a `CAMetalLayer` left at its default
/// `contentsScale = 1.0` renders a blurry, effectively-downscaled image
/// even though the Rust side correctly renders at full physical-pixel
/// resolution — one of the most common CAMetalLayer gotchas. Root-caused
/// and fixed 2026-07-08 after a direct visual report of blurry text.
final class MetalView: UIView {
    override class var layerClass: AnyClass { CAMetalLayer.self }

    override init(frame: CGRect) {
        super.init(frame: frame)
        (layer as! CAMetalLayer).contentsScale = UIScreen.main.scale
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
        (layer as! CAMetalLayer).contentsScale = UIScreen.main.scale
    }
}

final class EngineViewController: UIViewController, UIKeyInput {
    private var engine: RscEngine?
    private var displayLink: CADisplayLink?

    // MARK: Soft keyboard (D116 Step 6). The Metal view isn't a text field, so
    // the OS shows no keyboard on its own. Adopt `UIKeyInput` and, each tick,
    // become/resign first responder to match the engine's focused text field
    // (`rsc_text_input_active`), configuring the layout from its keyboard-type
    // hint. Keystrokes are forwarded back through `rsc_engine_input`.
    override var canBecomeFirstResponder: Bool { true }
    var keyboardType: UIKeyboardType = .default
    var hasText: Bool { true }

    private func sendKey(_ key: UInt32) {
        guard let engine else { return }
        var e = RscInputEvent(
            kind: RSC_EVENT_KEY_DOWN, x: 0, y: 0, button: 0,
            key: key, character: 0, width: 0, height: 0, delta_x: 0, delta_y: 0
        )
        withUnsafePointer(to: &e) { rsc_engine_input(engine, $0, 1) }
    }

    func insertText(_ text: String) {
        guard let engine else { return }
        for scalar in text.unicodeScalars {
            // Return and Tab are SPECIAL keys, not literal text: the engine
            // treats them as newline/submit and focus-traversal via KeyDown,
            // and drops control chars from the Text path — so forward them as
            // key events (matching what the desktop winit host sends).
            switch scalar {
            case "\n", "\r": sendKey(RSC_KEY_ENTER)
            case "\t":       sendKey(RSC_KEY_TAB)
            default:
                var e = RscInputEvent(
                    kind: RSC_EVENT_TEXT, x: 0, y: 0, button: 0,
                    key: 0, character: scalar.value, width: 0, height: 0, delta_x: 0, delta_y: 0
                )
                withUnsafePointer(to: &e) { rsc_engine_input(engine, $0, 1) }
            }
        }
    }

    func deleteBackward() {
        sendKey(RSC_KEY_BACKSPACE)
    }

    private func uiKeyboardType(for hint: UInt32) -> UIKeyboardType {
        switch hint {
        case 1:  return .emailAddress // RSC_KEYBOARD_EMAIL
        case 2:  return .numberPad    // RSC_KEYBOARD_NUMERIC
        case 3:  return .URL          // RSC_KEYBOARD_URL
        case 4:  return .phonePad     // RSC_KEYBOARD_PHONE
        default: return .default
        }
    }

    /// Show/hide/reconfigure the OS keyboard to match the focused field.
    private func syncSoftKeyboard() {
        if rsc_text_input_active() != 0 {
            let want = uiKeyboardType(for: rsc_focused_keyboard_type())
            if want != keyboardType {
                keyboardType = want
                if isFirstResponder { reloadInputViews() }
            }
            if !isFirstResponder { becomeFirstResponder() }
        } else if isFirstResponder {
            resignFirstResponder()
        }
    }

    override func loadView() {
        view = MetalView(frame: UIScreen.main.bounds)
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        let scale = Float(view.contentScaleFactor)
        let width = UInt32(view.bounds.width * CGFloat(scale))
        let height = UInt32(view.bounds.height * CGFloat(scale))
        let viewPtr = Unmanaged.passUnretained(view).toOpaque()
        engine = rsc_engine_init(viewPtr, width, height, scale)

        let link = CADisplayLink(target: self, selector: #selector(tick))
        link.add(to: .main, forMode: .default)
        displayLink = link
        runSyncDispatchSelfTest()

        // MARK: App lifecycle -> RSC_EVENT_LIFECYCLE_* (D110 Phase 29
        // Step 1). UIApplication notifications rather than AppDelegate/
        // SceneDelegate plumbing — this controller owns the engine handle,
        // so no cross-object wiring is needed. The Rust side applies these
        // immediately (not on the next frame): the display link pauses in
        // background, so a frame-queued Background event would only be
        // seen on resume.
        let nc = NotificationCenter.default
        nc.addObserver(self, selector: #selector(lifecycleActive),
                       name: UIApplication.didBecomeActiveNotification, object: nil)
        nc.addObserver(self, selector: #selector(lifecycleInactive),
                       name: UIApplication.willResignActiveNotification, object: nil)
        nc.addObserver(self, selector: #selector(lifecycleBackground),
                       name: UIApplication.didEnterBackgroundNotification, object: nil)
        nc.addObserver(self, selector: #selector(lifecycleSuspended),
                       name: UIApplication.willTerminateNotification, object: nil)
    }

    @objc private func lifecycleActive() { sendLifecycle(RSC_EVENT_LIFECYCLE_ACTIVE) }
    @objc private func lifecycleInactive() { sendLifecycle(RSC_EVENT_LIFECYCLE_INACTIVE) }
    @objc private func lifecycleBackground() { sendLifecycle(RSC_EVENT_LIFECYCLE_BACKGROUND) }
    @objc private func lifecycleSuspended() { sendLifecycle(RSC_EVENT_LIFECYCLE_SUSPENDED) }

    private func sendLifecycle(_ kind: UInt32) {
        guard let engine else { return }
        var event = RscInputEvent(
            kind: kind, x: 0, y: 0, button: 0,
            key: 0, character: 0, width: 0, height: 0, delta_x: 0, delta_y: 0
        )
        withUnsafePointer(to: &event) { rsc_engine_input(engine, $0, 1) }
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        guard let engine else { return }
        let scale = Float(view.contentScaleFactor)
        let width = UInt32(view.bounds.width * CGFloat(scale))
        let height = UInt32(view.bounds.height * CGFloat(scale))
        let insets = view.safeAreaInsets
        rsc_engine_resize(
            engine, width, height, scale,
            Float(insets.top), Float(insets.right), Float(insets.bottom), Float(insets.left)
        )
    }

    @objc private func tick() {
        guard let engine else { return }
        rsc_engine_frame(engine)
        pollPlatformChannel()
        syncSoftKeyboard()
    }

    /// The host's ONE per-frame poll for outgoing Platform Channel calls
    /// (D127) — push-permission discovery included, alongside anything an
    /// app registers itself. Recognizes `"rosace/push"` unconditionally
    /// (every app already carries push-permission polling, so there's no
    /// new per-app cost); THIS APP additionally recognizes its own two demo
    /// channels ("device" and camera) — an app wanting its own channel adds
    /// a case here the same way, for its own channel name.
    private func pollPlatformChannel() {
        guard let ptr = rsc_platform_channel_take_outgoing() else { return }
        defer { rsc_string_free(ptr) }
        guard let data = String(cString: ptr).data(using: .utf8),
              let calls = try? JSONSerialization.jsonObject(with: data) as? [[String: Any]] else { return }
        for call in calls {
            guard let channel = call["channel"] as? String, let method = call["method"] as? String,
                  let callId = (call["call_id"] as? NSNumber)?.uint64Value else { continue }
            switch (channel, method) {
            case ("rosace/push", "requestPermission"):
                requestPushPermission()
            case ("rosace/camera", "requestPermission"):
                requestCameraPermission()
            case ("dev.rosace.platformchanneldemo/device", "getOsVersion"):
                // Fast + synchronous in practice, but still goes through the
                // async path Rust-side (invoke_method) — the RIGHT default
                // for any native call, since native "could" take a moment
                // even when today it doesn't. Answered as a JSON string
                // literal (quoted), matching how Rust's `Value::to_string()`
                // expects the result_json it'll parse.
                let version = "iOS \(UIDevice.current.systemVersion)"
                let json = "\"\(version)\""
                json.withCString { rsc_platform_channel_report_result(callId, $0) }
            default:
                break
            }
        }
    }

    /// Real OS permission prompt + APNs registration. The result flows back
    /// through `rsc_push_permission_report_result`; a device token (if
    /// registration succeeds — it can legitimately fail without an
    /// aps-environment entitlement) arrives via AppDelegate's
    /// `didRegisterForRemoteNotificationsWithDeviceToken`.
    private func requestPushPermission() {
        UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .badge, .sound]) { granted, _ in
            rsc_push_permission_report_result(granted ? 1 : 0)
            if granted {
                DispatchQueue.main.async {
                    UIApplication.shared.registerForRemoteNotifications()
                }
            }
        }
    }

    /// The real `AVCaptureDevice` permission prompt — this app-specific
    /// channel is what `Demo 2` in the Platform Channel screen (Rust side:
    /// `screens/platform_channel.rs`) is actually exercising. Requires
    /// `NSCameraUsageDescription` in Info.plist (added for this demo only —
    /// see the module doc on why the shared generator doesn't add this
    /// unconditionally to every app).
    private func requestCameraPermission() {
        AVCaptureDevice.requestAccess(for: .video) { granted in
            rsc_camera_permission_report_result(granted ? 1 : 0)
        }
    }

    /// Proves the REVERSE direction (native calls Rust, synchronously) for
    /// real — `dev.rosace.platformchanneldemo::register_platform_channels`
    /// registered a handler for `MATH_CHANNEL`'s "add" method; this calls
    /// straight into it and logs what came back. Run once at launch so you
    /// can see the round trip in Xcode's console without any native UI
    /// needed to trigger it — see `Demo 3` in the Platform Channel screen.
    private func runSyncDispatchSelfTest() {
        let result = "dev.rosace.platformchanneldemo/math".withCString { channel in
            "add".withCString { method in
                "[2,3]".withCString { args in
                    rsc_platform_channel_dispatch(channel, method, args)
                }
            }
        }
        guard let result else { return }
        defer { rsc_string_free(result) }
        NSLog("Platform Channel self-test: add([2,3]) -> \(String(cString: result))")
    }

    // MARK: Touch -> MouseDown/MouseMove/MouseUp (same convention the
    // existing winit `Touch` handling and `RscInputEventFfi` conversion use
    // — no separate touch event kind needed).

    private func send(kind: UInt32, touches: Set<UITouch>) {
        guard let engine, let touch = touches.first else { return }
        let p = touch.location(in: view)
        var event = RscInputEvent(
            kind: kind, x: Float(p.x), y: Float(p.y), button: RSC_BUTTON_LEFT,
            key: 0, character: 0, width: 0, height: 0, delta_x: 0, delta_y: 0
        )
        withUnsafePointer(to: &event) { rsc_engine_input(engine, $0, 1) }
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        send(kind: RSC_EVENT_MOUSE_DOWN, touches: touches)
    }

    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        send(kind: RSC_EVENT_MOUSE_MOVE, touches: touches)
    }

    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        send(kind: RSC_EVENT_MOUSE_UP, touches: touches)
    }

    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        send(kind: RSC_EVENT_MOUSE_UP, touches: touches)
    }

    deinit {
        displayLink?.invalidate()
        if let engine { rsc_engine_shutdown(engine) }
    }
}
