//! Drives the ROSACE engine through the `rosace-ffi` C boundary
//! (`rosace-ffi/include/rsc_engine.h`) — a CAMetalLayer-backed view,
//! init/resize/input/frame calls, and real `UIView.safeAreaInsets` feeding
//! `rosace_core::SafeArea` (replacing the old winit outer/inner-size
//! workaround from Phase 20-22).

import UIKit
import QuartzCore
import UserNotifications

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

@_silgen_name("rsc_text_input_active")
func rsc_text_input_active() -> UInt8

@_silgen_name("rsc_focused_keyboard_type")
func rsc_focused_keyboard_type() -> UInt32

// D127 "environment" track — live OS brightness/accessibility push, same
// shape as `rsc_engine_resize`'s safe-area push above.
@_silgen_name("rsc_engine_set_media_query")
func rsc_engine_set_media_query(
    _ engine: RscEngine?, _ isDark: UInt8, _ textScale: Float,
    _ boldText: UInt8, _ reduceMotion: UInt8, _ always24HourFormat: UInt8
)

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

// Accessibility (D132): the engine's semantic tree as JSON, pulled on
// demand. See `MetalView`'s UIAccessibilityContainer conformance below.
@_silgen_name("rsc_engine_semantics_json")
func rsc_engine_semantics_json(_ engine: RscEngine?) -> UnsafeMutablePointer<CChar>?

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
/// One node of the engine's semantic tree, decoded from the FFI JSON.
private struct RscSemanticNode: Decodable {
    let id: UInt64?
    let role: String
    let label: String?
    let value: String?
    let bounds: RscBounds?
    let children: [RscSemanticNode]

    struct RscBounds: Decodable { let x: Float; let y: Float; let w: Float; let h: Float }
}

final class MetalView: UIView {
    override class var layerClass: AnyClass { CAMetalLayer.self }

    /// Supplies the engine's semantic tree as JSON. Set by
    /// `EngineViewController`, which owns the engine pointer.
    var semanticsJSONProvider: (() -> String?)?

    override init(frame: CGRect) {
        super.init(frame: frame)
        (layer as! CAMetalLayer).contentsScale = UIScreen.main.scale
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
        (layer as! CAMetalLayer).contentsScale = UIScreen.main.scale
    }

    // ── Accessibility (D132) ────────────────────────────────────────────
    //
    // ROSACE paints every pixel into this one CAMetalLayer, so without the
    // bridge below VoiceOver sees a single blank rectangle. UIKit asks for
    // `accessibilityElements` only while VoiceOver is actually inspecting,
    // so the engine is never serialized in the common case — which is why
    // the Rust side exposes this as a PULL rather than pushing each frame.
    //
    // The container must not itself be an element, or VoiceOver stops at
    // the container and never reaches the children.
    override var isAccessibilityElement: Bool {
        get { false }
        set { }
    }

    /// Cached so UIKit's retained element references stay alive.
    ///
    /// Rebuilding on every getter call returns fresh objects each time; the
    /// ones UIKit already handed to VoiceOver then go stale, and the
    /// Accessibility Inspector reports Label/Traits as None on an element
    /// whose header still shows the right name. Keyed on the JSON so the
    /// tree is only re-decoded when it actually changed.
    private var cachedJSON: String?
    private var cachedElements: [UIAccessibilityElement] = []

    override var accessibilityElements: [Any]? {
        get {
            guard let json = semanticsJSON(), !json.isEmpty else { return nil }
            if json != cachedJSON {
                cachedJSON = json
                cachedElements = buildElements(from: json)
            }
            return cachedElements.isEmpty ? nil : cachedElements
        }
        set { }
    }

    private func semanticsJSON() -> String? { semanticsJSONProvider?() }

    private func buildElements(from json: String) -> [UIAccessibilityElement] {
        guard let data = json.data(using: .utf8),
              let root = try? JSONDecoder().decode(RscSemanticNode.self, from: data)
        else { return [] }
        var out: [UIAccessibilityElement] = []
        appendElements(from: root, into: &out)
        return out
    }

    /// Flattens the tree into the linear list VoiceOver swipes through.
    ///
    /// Two rules, both learned from the Accessibility Inspector:
    ///
    /// 1. **An interactive control speaks for its own subtree.** A Button's
    ///    node and the Text inside it carry the same label, so emitting both
    ///    produced two elements stacked on one rect. The control wins and we
    ///    stop descending.
    /// 2. **Containers are emitted AFTER their children.** An AppBar declares
    ///    a heading spanning the whole bar, which contains the Back and Light
    ///    buttons. Emitting the container first put a full-width element on
    ///    top of them, so only the title was reachable. Overlapping frames are
    ///    legal; order decides priority, so children go in first and the
    ///    container still gets announced rather than being dropped.
    private func appendElements(from node: RscSemanticNode, into out: inout [UIAccessibilityElement]) {
        let speaks = (node.label?.isEmpty == false) || (node.value?.isEmpty == false)

        if speaks, isInteractive(node.role), let b = node.bounds {
            out.append(makeElement(node, b))
            return
        }
        for child in node.children {
            appendElements(from: child, into: &out)
        }
        if speaks, let b = node.bounds {
            out.append(makeElement(node, b))
        }
    }

    private func makeElement(_ node: RscSemanticNode, _ b: RscSemanticNode.RscBounds) -> UIAccessibilityElement {
        let element = UIAccessibilityElement(accessibilityContainer: self)
        element.accessibilityLabel = node.label
        element.accessibilityValue = node.value
        element.accessibilityTraits = traits(for: node.role)
        // Rust reports LOGICAL, view-relative pixels; UIKit wants screen
        // coordinates. UIAccessibility does the conversion (including the
        // scale factor), so we must not pre-multiply — the desktop bridge
        // shipped every element at half size by doing exactly that.
        let local = CGRect(x: CGFloat(b.x), y: CGFloat(b.y),
                           width: CGFloat(b.w), height: CGFloat(b.h))
        element.accessibilityFrame = UIAccessibility.convertToScreenCoordinates(local, in: self)
        return element
    }

    /// Roles that represent a control the user operates, rather than content
    /// or grouping. These stop the descent (rule 1 above).
    private func isInteractive(_ role: String) -> Bool {
        switch role {
        case "button", "checkbox", "radio", "switch", "textinput",
             "link", "slider", "tab", "menuitem":
            return true
        default:
            return false
        }
    }

    /// Maps ROSACE roles onto VoiceOver traits. Names must match
    /// `rosace-ffi`'s `role_name` exactly — that function spells them out
    /// literally so this mapping cannot drift with a Rust-side rename.
    private func traits(for role: String) -> UIAccessibilityTraits {
        switch role {
        case "button":      return .button
        case "link":        return .link
        case "heading":     return .header
        case "image":       return .image
        case "textinput":   return .searchField
        case "slider":      return .adjustable
        case "progressbar": return .updatesFrequently
        case "alert":       return .staticText
        case "tab":         return .button
        case "menuitem":    return .button
        // checkbox/radio/switch have no dedicated trait; VoiceOver conveys
        // their on/off state through accessibilityValue, which the engine
        // already supplies, so `.none` here is correct rather than lossy.
        default:            return .none
        }
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

        // Accessibility (D132): hand the view a way to pull the semantic
        // tree. UIKit only calls this while VoiceOver is inspecting, so an
        // app with no screen reader running never serializes anything.
        // `unowned` rather than a strong capture — the view is owned by this
        // controller, so a strong reference here would be a retain cycle.
        if let metalView = view as? MetalView {
            metalView.semanticsJSONProvider = { [unowned self] in
                guard let engine = self.engine,
                      let ptr = rsc_engine_semantics_json(engine) else { return nil }
                defer { rsc_string_free(ptr) }
                return String(cString: ptr)
            }
        }

        let link = CADisplayLink(target: self, selector: #selector(tick))
        link.add(to: .main, forMode: .default)
        displayLink = link

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

        // Bold Text / Reduce Motion are `UIAccessibility` settings, not
        // `UITraitCollection` traits — they don't fire `traitCollectionDidChange`,
        // so they need their own notifications.
        nc.addObserver(self, selector: #selector(syncMediaQuery),
                       name: UIAccessibility.boldTextStatusDidChangeNotification, object: nil)
        nc.addObserver(self, selector: #selector(syncMediaQuery),
                       name: UIAccessibility.reduceMotionStatusDidChangeNotification, object: nil)
        syncMediaQuery()
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

    // MARK: Environment (D127) — OS brightness/Dynamic-Type/accessibility,
    // pushed live via `rsc_engine_set_media_query` whenever the OS reports a
    // change, mirroring how safe-area is pushed on every layout pass above.

    /// Apple's documented default point size for `UIFont.TextStyle.body` at
    /// each `UIContentSizeCategory`, expressed as a ratio against `.large`
    /// (17pt — the non-accessibility system default) — the standard
    /// technique for turning Dynamic Type's category enum into the single
    /// float multiplier `rosace_core::MediaQuery.text_scale` expects.
    private func textScale(for category: UIContentSizeCategory) -> Float {
        switch category {
        case .extraSmall:                       return 14.0 / 17.0
        case .small:                             return 15.0 / 17.0
        case .medium:                            return 16.0 / 17.0
        case .large:                             return 1.0
        case .extraLarge:                        return 19.0 / 17.0
        case .extraExtraLarge:                   return 21.0 / 17.0
        case .extraExtraExtraLarge:              return 23.0 / 17.0
        case .accessibilityMedium:               return 28.0 / 17.0
        case .accessibilityLarge:                return 33.0 / 17.0
        case .accessibilityExtraLarge:           return 40.0 / 17.0
        case .accessibilityExtraExtraLarge:      return 47.0 / 17.0
        case .accessibilityExtraExtraExtraLarge: return 53.0 / 17.0
        default:                                 return 1.0
        }
    }

    @objc private func syncMediaQuery() {
        guard let engine else { return }
        let isDark = traitCollection.userInterfaceStyle == .dark
        let scale = textScale(for: traitCollection.preferredContentSizeCategory)
        rsc_engine_set_media_query(
            engine,
            isDark ? 1 : 0,
            scale,
            UIAccessibility.isBoldTextEnabled ? 1 : 0,
            UIAccessibility.isReduceMotionEnabled ? 1 : 0,
            0 // always_24_hour_format: no clean UIKit source — left undetected on iOS for now
        )
    }

    /// Fires live for BOTH userInterfaceStyle (dark mode) and
    /// preferredContentSizeCategory (Dynamic Type) changes — both are
    /// `UITraitCollection` traits.
    override func traitCollectionDidChange(_ previousTraitCollection: UITraitCollection?) {
        super.traitCollectionDidChange(previousTraitCollection)
        syncMediaQuery()
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
    /// new per-app cost); an app wanting its own channel (camera, a custom
    /// native SDK, …) adds a case here for its own channel name.
    private func pollPlatformChannel() {
        guard let ptr = rsc_platform_channel_take_outgoing() else { return }
        defer { rsc_string_free(ptr) }
        guard let data = String(cString: ptr).data(using: .utf8),
              let calls = try? JSONSerialization.jsonObject(with: data) as? [[String: Any]] else { return }
        for call in calls {
            guard let channel = call["channel"] as? String, let method = call["method"] as? String else { continue }
            if channel == "rosace/push" && method == "requestPermission" {
                requestPushPermission()
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
