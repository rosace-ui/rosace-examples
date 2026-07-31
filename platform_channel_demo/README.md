# platform_channel_demo

A live, working showcase of ROSACE's **Platform Channel** — a generic,
named, bidirectional bridge between your Rust app code and native
platform code (Swift/Kotlin), in the style of Flutter's own
`MethodChannel`. Use this as the reference for wiring your own app up to
any native SDK or OS API ROSACE doesn't have first-class support for.

## Run it

```sh
rsc run --mac           # macOS
rsc run --target ios    # iOS simulator (drives real xcodebuild)
```

For Android, open `android/` in Android Studio, or:
```sh
cd android && ./gradlew installDebug
```

> **Platform note**: Platform Channel's native side only exists on iOS and
> Android. On macOS/Windows/Linux the async calls below will sit in
> "Pending" forever — nothing polls the outgoing-call queue there yet.
> That's expected, not a bug.

## What to look for

Open the **Platform Channel** screen from the home screen. Three demos,
each showing a different half of the mechanism:

1. **Device Info** — tap "Get OS Version". Rust calls native
   asynchronously (`rosace_ffi::invoke_method`); the UI reactively shows
   `Pending` → the real OS version, with zero polling code on the Rust
   side. This is the common direction: **Rust asks native for something.**

2. **Camera Permission** — tap "Request Camera Permission" to see a REAL
   `AVCaptureDevice`/Android runtime-permission prompt. This uses
   `rosace_ffi::request_camera()`, a capability already built into ROSACE
   — this demo app's own native code (`ios/App/EngineViewController.swift`,
   `android/.../MainActivity.kt`) is what wires it up, since camera access
   is opt-in per app (see the comments there for why it's not baked into
   every generated app by default).

3. **Sync Dispatch Self-Test** — nothing to tap here. At launch, this
   app's native code calls straight into a Rust handler
   (`lib.rs::app_init`, registered on the `"dev.rosace.platformchanneldemo/math"`
   channel) and gets an answer back in one blocking call — **native asks
   Rust for something**, the reverse direction. Check Xcode's console (or
   `adb logcat`, filter `rosace`) for:
   ```
   Platform Channel self-test: add([2,3]) -> 5
   ```

## Where the code lives

| What | File |
|---|---|
| The demo screen (heavily commented — start here) | `src/screens/platform_channel.rs` |
| App-level startup / handler registration | `src/lib.rs` (`app_init`) |
| iOS native bridge | `ios/App/EngineViewController.swift` |
| Android native bridge | `android/app/src/main/java/dev/rosace/platformchanneldemo/MainActivity.kt` |
| The Platform Channel primitive itself (framework side) | `rosace-ffi/src/platform_channel/{dispatch,outgoing}.rs` in the main ROSACE repo |

## A gotcha worth knowing (and why it's fixed here)

Mobile's native entry points (`rsc_engine_init` on iOS, `nativeInit` on
Android) construct the engine directly — they never call the desktop/web
`launch()` function. Any one-time Rust-side setup (like registering a
Platform Channel handler) has to happen in `app_init()` instead, which
`rsc new`'s generator now calls from *every* platform entry point. Putting
setup code only in `launch()` will silently never run on iOS/Android —
found and fixed live while building this exact demo.
