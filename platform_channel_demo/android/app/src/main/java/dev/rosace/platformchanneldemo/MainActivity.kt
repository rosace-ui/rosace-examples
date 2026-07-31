package dev.rosace.platformchanneldemo

import android.app.Activity
import android.content.Context
import android.os.Bundle
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
    }

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
        init { System.loadLibrary("platform_channel_demo") }
        private const val CAMERA_PERMISSION_REQUEST_CODE = 1001
    }

    private external fun nativeInit(surface: Surface, width: Int, height: Int, scale: Float): Long
    private external fun nativeResize(
        handle: Long, width: Int, height: Int, scale: Float,
        safeTop: Float, safeRight: Float, safeBottom: Float, safeLeft: Float,
    )
    private external fun nativeTouch(handle: Long, kind: Int, x: Float, y: Float)
    private external fun nativeKey(handle: Long, key: Int)
    private external fun nativeText(handle: Long, character: Int)
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
    // Camera is app-specific, not part of the shared generator (mirrors
    // ios/App/EngineViewController.swift's rsc_camera_permission_report_result
    // — see ffi.rs's doc on why the shared template doesn't add this
    // unconditionally to every app).
    private external fun nativeCameraPermissionReportResult(granted: Int)
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
        )
        surfaceView.holder.addCallback(this)
        setContentView(surfaceView)
        runSyncDispatchSelfTest()
    }

    /// Proves the REVERSE direction (native calls Rust, synchronously) for
    /// real — `dev.rosace.platformchanneldemo::register_platform_channels`
    /// registered a handler for the math channel's "add" method; this calls
    /// straight into it and logs what came back. Run once at launch so you
    /// can see the round trip in `adb logcat` without any native UI needed
    /// to trigger it — see `Demo 3` in the Platform Channel screen.
    private fun runSyncDispatchSelfTest() {
        val result = nativePlatformChannelDispatch("dev.rosace.platformchanneldemo/math", "add", "[2,3]")
        android.util.Log.i("rosace", "Platform Channel self-test: add([2,3]) -> $result")
    }

    /// The real Android runtime-permission prompt (API 23+) — this
    /// app-specific channel is what `Demo 2` in the Platform Channel screen
    /// (Rust side: `screens/platform_channel.rs`) is actually exercising.
    /// Requires the `CAMERA` manifest permission (added for this demo only
    /// — see `ffi.rs`'s doc on why the shared generator doesn't add this
    /// unconditionally to every app).
    private fun requestCameraPermission() {
        if (checkSelfPermission(android.Manifest.permission.CAMERA)
            == android.content.pm.PackageManager.PERMISSION_GRANTED) {
            nativeCameraPermissionReportResult(1)
        } else {
            requestPermissions(arrayOf(android.Manifest.permission.CAMERA), CAMERA_PERMISSION_REQUEST_CODE)
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int, permissions: Array<out String>, grantResults: IntArray,
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == CAMERA_PERMISSION_REQUEST_CODE) {
            val granted = grantResults.isNotEmpty() &&
                grantResults[0] == android.content.pm.PackageManager.PERMISSION_GRANTED
            nativeCameraPermissionReportResult(if (granted) 1 else 0)
        }
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
    /// (D127). Android push permission (`POST_NOTIFICATIONS`, API 33+) was
    /// never wired before this demo — building real Android push support is
    /// separate scope, so `"rosace/push"` still isn't recognized. THIS APP
    /// recognizes its own two demo channels ("device" and camera), the same
    /// way any app adds a case here for its own channel name.
    private fun pollPlatformChannel() {
        val json = nativeTakeOutgoingPlatformCalls() ?: return
        val calls = try { org.json.JSONArray(json) } catch (e: org.json.JSONException) { return }
        for (i in 0 until calls.length()) {
            val call = calls.optJSONObject(i) ?: continue
            val channel = call.optString("channel")
            val method = call.optString("method")
            val callId = call.optLong("call_id")
            when {
                channel == "rosace/camera" && method == "requestPermission" ->
                    requestCameraPermission()
                channel == "dev.rosace.platformchanneldemo/device" && method == "getOsVersion" -> {
                    // Fast + synchronous in practice, but still goes through
                    // the async path Rust-side (invoke_method) — the RIGHT
                    // default for any native call, since native "could" take
                    // a moment even when today it doesn't. Answered as a
                    // JSON string literal (quoted), matching how Rust's
                    // `Value::to_string()` expects the result_json it parses.
                    val version = "Android ${android.os.Build.VERSION.RELEASE}"
                    nativePlatformChannelReportResult(callId, org.json.JSONObject.quote(version))
                }
                else -> android.util.Log.d("rosace", "Platform Channel call: $channel/$method (unhandled)")
            }
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
