package dev.rosace.showcase

import android.app.Activity
import android.content.Context
import android.content.res.Configuration
import android.os.Bundle
import android.provider.Settings
import android.text.InputType
import android.view.Choreographer
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView
import android.view.inputmethod.BaseInputConnection
import android.view.inputmethod.EditorInfo
import android.view.inputmethod.InputConnection
import android.view.inputmethod.InputMethodManager
import android.graphics.Rect
import android.view.accessibility.AccessibilityNodeInfo
import android.view.accessibility.AccessibilityNodeProvider
import org.json.JSONObject

/// A `SurfaceView` that can receive the soft keyboard's `InputConnection`
/// (D116 Step 6, Android). A plain `SurfaceView` isn't a text editor, so the
/// OS never offers to show a keyboard for it — opting in via
/// `onCheckIsTextEditor`/`onCreateInputConnection` is the same mechanism a
/// custom text-editing widget uses, mirroring iOS's `UIKeyInput` conformance
/// on the Metal view. Typed characters and special keys (Backspace/Enter/Tab)
/// are forwarded out through the two callbacks rather than calling the JNI
/// bridge directly, so this view has no engine-handle knowledge of its own.
private class EngineSurfaceView(
    context: Context,
    private val onText: (Int) -> Unit,
    private val onKey: (Int) -> Unit,
    /// Supplies the engine's semantic tree as JSON (D132). A callback, like
    /// `onText`/`onKey`, so this view keeps no engine-handle knowledge.
    private val semanticsJson: () -> String?,
) : SurfaceView(context) {
    // RSC_KEY_* (rosace_ffi::event) — Enter/Tab/Backspace are commands, never
    // literal text, same convention iOS's insertText special-cases them with.
    private val keyEnter = 0
    private val keyBackspace = 3
    private val keyTab = 4

    var keyboardInputType: Int = InputType.TYPE_CLASS_TEXT

    init {
        isFocusable = true
        isFocusableInTouchMode = true
        // Without this the view never reaches the accessibility tree at all:
        // a SurfaceView carries no text or contentDescription, so the default
        // IMPORTANT_FOR_ACCESSIBILITY_AUTO resolves to "not important" and
        // `getAccessibilityNodeProvider` is never called. Verified with
        // `uiautomator dump`, which showed only the parent FrameLayout (D132).
        importantForAccessibility = IMPORTANT_FOR_ACCESSIBILITY_YES
        contentDescription = null
    }

    // -- Accessibility (D132) ------------------------------------------
    //
    // ROSACE draws every pixel into this one SurfaceView, so without the
    // provider below TalkBack sees a single unlabelled rectangle.
    //
    // Android's model is NOT iOS's. UIKit takes an array of element objects;
    // Android asks for one node at a time by an Int "virtual view id" and
    // expects parent/child links expressed as ids. So the tree is flattened
    // once per query and the LIST INDEX is used as the id — our own semantic
    // ids are u64 and would not fit an Int.
    //
    // Pull, not push: these methods are only called while an accessibility
    // service is exploring, so TalkBack-off costs nothing.

    private class A11yNode(
        val label: String,
        val role: String,
        val bounds: Rect?,
        val children: MutableList<Int> = mutableListOf(),
        var parent: Int = AccessibilityNodeProvider.HOST_VIEW_ID,
    )

    private fun flatten(): List<A11yNode> {
        val json = semanticsJson() ?: return emptyList()
        val out = mutableListOf<A11yNode>()
        try {
            walk(JSONObject(json), out, AccessibilityNodeProvider.HOST_VIEW_ID)
        } catch (e: Exception) {
            return emptyList()
        }
        return out
    }

    /// Same two rules as the iOS bridge, for the same reasons: an
    /// interactive control speaks for its subtree (otherwise a Button and
    /// the Text inside it both become nodes on one rect), and a container is
    /// emitted AFTER its children so its full-width rect does not occlude
    /// them in hit-testing order.
    private fun walk(node: JSONObject, out: MutableList<A11yNode>, parent: Int) {
        // `optString` returns the literal string "null" for a JSON null —
        // org.json's long-standing gotcha. Read through `isNull` or TalkBack
        // announces a phantom node that literally says "null" (seen in a
        // uiautomator dump before this guard).
        val rawLabel = if (node.isNull("label")) "" else node.optString("label", "")
        val rawValue = if (node.isNull("value")) "" else node.optString("value", "")
        val label = rawLabel.ifEmpty { rawValue }
        val role = node.optString("role", "unknown")
        val speaks = label.isNotEmpty()
        val kids = node.optJSONArray("children")

        if (speaks && isInteractive(role)) {
            out.add(makeNode(node, label, role, parent))
            return
        }
        val childIds = mutableListOf<Int>()
        if (kids != null) {
            for (i in 0 until kids.length()) {
                val before = out.size
                walk(kids.getJSONObject(i), out, parent)
                for (j in before until out.size) {
                    if (out[j].parent == parent) childIds.add(j)
                }
            }
        }
        if (speaks) {
            out.add(makeNode(node, label, role, parent))
        }
    }

    private fun makeNode(node: JSONObject, label: String, role: String, parent: Int): A11yNode {
        val b = node.optJSONObject("bounds")
        var rect: Rect? = null
        if (b != null) {
            // Rust reports LOGICAL, view-relative px. AccessibilityNodeInfo
            // wants PHYSICAL screen px, so scale by density and offset by the
            // view's position — the mirror of the conversion iOS does with
            // UIAccessibility.convertToScreenCoordinates.
            val d = resources.displayMetrics.density
            val loc = IntArray(2)
            getLocationOnScreen(loc)
            val x = (b.optDouble("x", 0.0) * d).toInt() + loc[0]
            val y = (b.optDouble("y", 0.0) * d).toInt() + loc[1]
            val w = (b.optDouble("w", 0.0) * d).toInt()
            val h = (b.optDouble("h", 0.0) * d).toInt()
            rect = Rect(x, y, x + w, y + h)
        }
        val n = A11yNode(label, role, rect)
        n.parent = parent
        return n
    }

    private fun isInteractive(role: String): Boolean = when (role) {
        "button", "checkbox", "radio", "switch", "textinput",
        "link", "slider", "tab", "menuitem" -> true
        else -> false
    }

    /// TalkBack derives the spoken role from the class name, the way it does
    /// for real framework widgets.
    private fun classNameFor(role: String): String = when (role) {
        "button", "menuitem", "tab" -> "android.widget.Button"
        "checkbox" -> "android.widget.CheckBox"
        "radio" -> "android.widget.RadioButton"
        "switch" -> "android.widget.Switch"
        "textinput" -> "android.widget.EditText"
        "image" -> "android.widget.ImageView"
        "slider", "progressbar" -> "android.widget.SeekBar"
        else -> "android.widget.TextView"
    }

    private val provider = object : AccessibilityNodeProvider() {
        override fun createAccessibilityNodeInfo(virtualViewId: Int): AccessibilityNodeInfo? {
            val nodes = flatten()
            if (virtualViewId == HOST_VIEW_ID) {
                val info = AccessibilityNodeInfo.obtain(this@EngineSurfaceView)
                onInitializeAccessibilityNodeInfo(info)
                // Only top-level nodes attach to the host; nested ones are
                // reached through their own parent.
                nodes.forEachIndexed { i, n ->
                    if (n.parent == HOST_VIEW_ID) info.addChild(this@EngineSurfaceView, i)
                }
                return info
            }
            val n = nodes.getOrNull(virtualViewId) ?: return null
            val info = AccessibilityNodeInfo.obtain(this@EngineSurfaceView, virtualViewId)
            info.className = classNameFor(n.role)
            info.text = n.label
            info.contentDescription = n.label
            info.packageName = context.packageName
            info.setParent(this@EngineSurfaceView)
            n.bounds?.let { info.setBoundsInScreen(it) }
            info.isVisibleToUser = true
            info.isEnabled = true
            if (isInteractive(n.role)) {
                info.isClickable = true
                info.isFocusable = true
                info.addAction(AccessibilityNodeInfo.ACTION_CLICK)
            }
            return info
        }

        override fun performAction(virtualViewId: Int, action: Int, arguments: Bundle?): Boolean {
            // Activation would have to route back into the engine's
            // hit-test/dispatch path, which this view deliberately has no
            // handle for. Named as a gap in D132 rather than silently
            // reporting success for something that did nothing.
            return false
        }

        override fun findFocus(focus: Int): AccessibilityNodeInfo? = null
    }

    override fun getAccessibilityNodeProvider(): AccessibilityNodeProvider = provider

    override fun onCheckIsTextEditor(): Boolean = true

    override fun onCreateInputConnection(outAttrs: EditorInfo): InputConnection {
        outAttrs.inputType = keyboardInputType
        outAttrs.imeOptions = EditorInfo.IME_FLAG_NO_EXTRACT_UI or EditorInfo.IME_FLAG_NO_FULLSCREEN
        return object : BaseInputConnection(this, false) {
            override fun commitText(text: CharSequence, newCursorPosition: Int): Boolean {
                text.codePoints().forEach { onText(it) }
                return true
            }

            override fun deleteSurroundingText(beforeLength: Int, afterLength: Int): Boolean {
                // Predictive-input/composing backspace arrives this way
                // instead of a KeyEvent — treat each deleted char the same
                // as a real Backspace keypress.
                repeat(beforeLength) { onKey(keyBackspace) }
                return true
            }

            override fun sendKeyEvent(event: KeyEvent): Boolean {
                if (event.action == KeyEvent.ACTION_DOWN) {
                    when (event.keyCode) {
                        KeyEvent.KEYCODE_DEL -> onKey(keyBackspace)
                        KeyEvent.KEYCODE_ENTER -> onKey(keyEnter)
                        KeyEvent.KEYCODE_TAB -> onKey(keyTab)
                    }
                }
                return super.sendKeyEvent(event)
            }
        }
    }
}

class MainActivity : Activity(), SurfaceHolder.Callback {

    companion object {
        init { System.loadLibrary("showcase") }
    }

    private external fun nativeInit(surface: Surface, width: Int, height: Int, scale: Float): Long
    private external fun nativeResize(
        handle: Long, width: Int, height: Int, scale: Float,
        safeTop: Float, safeRight: Float, safeBottom: Float, safeLeft: Float,
    )
    // D127 "environment" track — live OS brightness/accessibility push,
    // called once from surfaceCreated and again from every
    // onConfigurationChanged.
    private external fun nativeSetMediaQuery(
        handle: Long, isDark: Boolean, textScale: Float,
        boldText: Boolean, reduceMotion: Boolean, always24HourFormat: Boolean,
    )
    private external fun nativeTouch(handle: Long, kind: Int, x: Float, y: Float)
    private external fun nativeKey(handle: Long, key: Int)
    private external fun nativeText(handle: Long, character: Int)
    private external fun nativeSemanticsJson(handle: Long): String?
    private external fun nativeTextInputActive(): Boolean
    private external fun nativeFocusedKeyboardType(): Int
    // Platform Channel (D127) — the generic bidirectional method-call bridge
    // to native code, mirroring the plain-C exports iOS's EngineViewController
    // declares via @_silgen_name. take_outgoing is the host's ONE per-frame
    // poll (see pollPlatformChannel below); dispatch is the reverse direction
    // (native calling a Rust-registered handler), included for completeness
    // even though this template doesn't call it itself.
    private external fun nativeTakeOutgoingPlatformCalls(): String?
    private external fun nativePlatformChannelReportResult(callId: Long, resultJson: String)
    private external fun nativePlatformChannelReportError(callId: Long, message: String)
    private external fun nativePlatformChannelDispatch(channel: String, method: String, argsJson: String): String?
    private external fun nativeLifecycle(handle: Long, kind: Int)
    private external fun nativeFrame(handle: Long)
    private external fun nativeShutdown(handle: Long)

    private var engineHandle: Long = 0
    private lateinit var surfaceView: EngineSurfaceView

    private val frameCallback = object : Choreographer.FrameCallback {
        override fun doFrame(frameTimeNanos: Long) {
            if (engineHandle != 0L) {
                nativeFrame(engineHandle)
                pollPlatformChannel()
                syncSoftKeyboard()
                Choreographer.getInstance().postFrameCallback(this)
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        surfaceView = EngineSurfaceView(
            this,
            onText = { c -> if (engineHandle != 0L) nativeText(engineHandle, c) },
            onKey = { k -> if (engineHandle != 0L) nativeKey(engineHandle, k) },
            // D132: TalkBack pulls the tree through here, only while an
            // accessibility service is actually exploring.
            semanticsJson = { if (engineHandle != 0L) nativeSemanticsJson(engineHandle) else null },
        )
        surfaceView.holder.addCallback(this)
        setContentView(surfaceView)
    }

    /// Show/hide/reconfigure the soft keyboard to match the engine's focused
    /// field (D116 Step 6) — the Android counterpart of iOS's
    /// `syncSoftKeyboard`, polled once per frame tick the same way.
    private fun uiInputTypeFor(hint: Int): Int = when (hint) {
        1 -> InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_EMAIL_ADDRESS // RSC_KEYBOARD_EMAIL
        2 -> InputType.TYPE_CLASS_NUMBER                                              // RSC_KEYBOARD_NUMERIC
        3 -> InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_URI           // RSC_KEYBOARD_URL
        4 -> InputType.TYPE_CLASS_PHONE                                              // RSC_KEYBOARD_PHONE
        else -> InputType.TYPE_CLASS_TEXT                                            // RSC_KEYBOARD_DEFAULT
    }

    private fun syncSoftKeyboard() {
        val imm = getSystemService(Context.INPUT_METHOD_SERVICE) as InputMethodManager
        if (nativeTextInputActive()) {
            val want = uiInputTypeFor(nativeFocusedKeyboardType())
            if (want != surfaceView.keyboardInputType) {
                surfaceView.keyboardInputType = want
                if (surfaceView.isFocused) imm.restartInput(surfaceView)
            }
            if (!surfaceView.isFocused) {
                surfaceView.requestFocus()
                imm.showSoftInput(surfaceView, InputMethodManager.SHOW_IMPLICIT)
            }
        } else if (surfaceView.isFocused) {
            imm.hideSoftInputFromWindow(surfaceView.windowToken, 0)
        }
    }

    /// The host's ONE per-frame poll for outgoing Platform Channel calls
    /// (D127). Unlike iOS (which already had a real push-permission flow to
    /// migrate onto this), Android push permission (`POST_NOTIFICATIONS`,
    /// API 33+) was never wired here — so `"rosace/push"` is deliberately
    /// NOT recognized below yet (a named follow-up, not a regression: there
    /// was nothing to preserve). An app wanting its own channel (camera, a
    /// custom native SDK, or building out real Android push support) adds a
    /// case here for its own channel name and reports back via
    /// `nativePlatformChannelReportResult`/`_ReportError`.
    private fun pollPlatformChannel() {
        val json = nativeTakeOutgoingPlatformCalls() ?: return
        val calls = try { org.json.JSONArray(json) } catch (e: org.json.JSONException) { return }
        for (i in 0 until calls.length()) {
            val call = calls.optJSONObject(i) ?: continue
            val channel = call.optString("channel")
            val method = call.optString("method")
            // (no built-in channels recognized yet — see the doc above;
            // logged so a custom channel's calls are visible during dev)
            android.util.Log.d("rosace", "Platform Channel call: $channel/$method (unhandled)")
        }
    }

    // App lifecycle -> RSC_EVENT_LIFECYCLE_* (D110 Phase 29 Step 1);
    // kinds match rsc_engine.h (8 = active, 9 = inactive, 10 = background).
    // Android has no reliable pre-kill callback, so SUSPENDED (11) is not
    // sent — onDestroy is not guaranteed to run. Applied immediately on
    // the Rust side, so onStop's event lands even though the Choreographer
    // callback has gone quiet by then.
    override fun onResume() {
        super.onResume()
        if (engineHandle != 0L) nativeLifecycle(engineHandle, 8)
    }

    override fun onPause() {
        super.onPause()
        if (engineHandle != 0L) nativeLifecycle(engineHandle, 9)
    }

    override fun onStop() {
        super.onStop()
        if (engineHandle != 0L) nativeLifecycle(engineHandle, 10)
    }

    override fun surfaceCreated(holder: SurfaceHolder) {
        val scale = resources.displayMetrics.density
        val width = surfaceView.width
        val height = surfaceView.height
        engineHandle = nativeInit(holder.surface, width, height, scale)
        Choreographer.getInstance().postFrameCallback(frameCallback)
        syncMediaQuery()
    }

    // MARK: Environment (D127) — OS brightness/font-scale/reduce-motion,
    // pushed live via nativeSetMediaQuery whenever the OS reports a change.
    // `android:configChanges` in AndroidManifest.xml lists `uiMode|fontScale`
    // so this Activity survives the change and gets the live callback below,
    // instead of being torn down and recreated (which would lose in-memory
    // engine state on every dark-mode toggle).

    /// No clean OS-wide "bold text everywhere" source on Android (unlike
    /// iOS's `UIAccessibility.isBoldTextEnabled`) — stays `false`, a
    /// documented platform gap (see `rosace_core::media_query`'s doc).
    private fun syncMediaQuery() {
        if (engineHandle == 0L) return
        val uiMode = resources.configuration.uiMode
        val isDark = (uiMode and Configuration.UI_MODE_NIGHT_MASK) == Configuration.UI_MODE_NIGHT_YES
        val textScale = resources.configuration.fontScale
        val reduceMotion = Settings.Global.getFloat(
            contentResolver, Settings.Global.ANIMATOR_DURATION_SCALE, 1f,
        ) == 0f
        val always24Hour = android.text.format.DateFormat.is24HourFormat(this)
        nativeSetMediaQuery(engineHandle, isDark, textScale, false, reduceMotion, always24Hour)
    }

    override fun onConfigurationChanged(newConfig: Configuration) {
        super.onConfigurationChanged(newConfig)
        syncMediaQuery()
    }

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
        if (engineHandle == 0L) return
        val scale = resources.displayMetrics.density
        // Basic safe-area: only the status bar height (systemWindowInsetTop),
        // not a full WindowInsets-driven cutout/gesture-nav treatment — a
        // known simplification (see .steering/CRATE_CONTRACTS.md Known
        // Issues), the Android counterpart of iOS's real UIView.safeAreaInsets
        // (Step 2) is follow-up work, not silently claimed equivalent here.
        nativeResize(engineHandle, width, height, scale, 0f, 0f, 0f, 0f)
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        if (engineHandle == 0L) return
        nativeShutdown(engineHandle)
        engineHandle = 0
    }

    override fun onTouchEvent(event: MotionEvent): Boolean {
        if (engineHandle == 0L) return false
        val kind = when (event.actionMasked) {
            MotionEvent.ACTION_DOWN -> 1
            MotionEvent.ACTION_MOVE -> 0
            MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> 2
            else -> return false
        }
        // MotionEvent x/y are PHYSICAL pixels; the engine hit-tests in LOGICAL
        // coordinates (like the desktop `position.x / scale_factor` path and
        // iOS's already-logical `touch.location(in: view)`). Divide by the same
        // density used for nativeResize — without this, taps on a hi-DPI screen
        // land at 2-3x the intended point and every click misses its target.
        val density = resources.displayMetrics.density
        nativeTouch(engineHandle, kind, event.x / density, event.y / density)
        return true
    }
}
