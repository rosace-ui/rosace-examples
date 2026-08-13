package dev.rosace.galleryall

import android.app.Activity
import android.os.Bundle
import android.view.Choreographer
import android.view.MotionEvent
import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView

class MainActivity : Activity(), SurfaceHolder.Callback {

    companion object {
        init { System.loadLibrary("gallery_all") }
    }

    private external fun nativeInit(surface: Surface, width: Int, height: Int, scale: Float): Long
    private external fun nativeResize(
        handle: Long, width: Int, height: Int, scale: Float,
        safeTop: Float, safeRight: Float, safeBottom: Float, safeLeft: Float,
    )
    private external fun nativeTouch(handle: Long, kind: Int, x: Float, y: Float)
    private external fun nativeLifecycle(handle: Long, kind: Int)
    private external fun nativeFrame(handle: Long)
    private external fun nativeShutdown(handle: Long)

    private var engineHandle: Long = 0
    private lateinit var surfaceView: SurfaceView

    private val frameCallback = object : Choreographer.FrameCallback {
        override fun doFrame(frameTimeNanos: Long) {
            if (engineHandle != 0L) {
                nativeFrame(engineHandle)
                Choreographer.getInstance().postFrameCallback(this)
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        surfaceView = SurfaceView(this)
        surfaceView.holder.addCallback(this)
        setContentView(surfaceView)
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
