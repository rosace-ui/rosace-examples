plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "dev.rosace.galleryall"
    compileSdk = 34

    defaultConfig {
        applicationId = "dev.rosace.galleryall"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"
        ndk {
            abiFilters += listOf("arm64-v8a")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

// Builds the Rust cdylib for the target ABI(s) and stages it into
// src/main/jniLibs/<abi>/ before Gradle's own resource-merge step picks it
// up via the jniLibs.srcDirs above — the Android counterpart to Step 2's
// Xcode PBXShellScriptBuildPhase. Verified: this task, followed by
// assembleDebug, produces a real .so-containing APK (see .steering/
// PHASE_24.md's Step 3 verification note); NDK path matches this machine's
// install and isn't yet configurable — a real per-project setup would read
// it from ANDROID_NDK_HOME, tracked as follow-up.
tasks.register("cargoBuildAndroid") {
    doLast {
        val abi = "arm64-v8a"
        val rustTriple = "aarch64-linux-android"
        // NDK root from the environment, not a hardcoded machine path —
        // ANDROID_NDK_HOME if set, else the newest version under
        // $ANDROID_HOME/ndk. Host-tag ("darwin-x86_64" etc.) still assumes
        // the NDK's own prebuilt-toolchain naming; only macOS/Linux/Windows
        // x86_64 hosts are handled, matching what this project has
        // actually been verified on (see .steering/CRATE_CONTRACTS.md
        // Known Issues) — ARM-host NDK layouts are a follow-up.
        val ndkHome = System.getenv("ANDROID_NDK_HOME")
            ?: File(System.getenv("ANDROID_HOME") ?: "${System.getProperty("user.home")}/Library/Android/sdk", "ndk")
                .listFiles()?.maxByOrNull { it.name }?.absolutePath
            ?: throw GradleException("Set ANDROID_NDK_HOME, or install an NDK under \$ANDROID_HOME/ndk")
        val hostTag = when {
            org.gradle.internal.os.OperatingSystem.current().isMacOsX -> "darwin-x86_64"
            org.gradle.internal.os.OperatingSystem.current().isLinux -> "linux-x86_64"
            else -> "windows-x86_64"
        }
        val minSdk = 24
        val toolchainBin = "$ndkHome/toolchains/llvm/prebuilt/$hostTag/bin"
        val linker = "$toolchainBin/aarch64-linux-android$minSdk-clang"
        // C/C++ compiler + archiver for the target. REQUIRED: several Rust
        // deps compile C — `ring` (rustls TLS, via networking), `rusqlite`'s
        // bundled SQLite (persistence), `ndk-sys`. Without these env vars the
        // `cc` crate looks for a bare `aarch64-linux-android-clang` (no API
        // level) that the NDK doesn't ship, and the build fails. (Phase 24's
        // Android build predated all the C-compiling deps, so the original
        // template only set the linker — this is the fix for that gap.)
        val cc = "$toolchainBin/aarch64-linux-android$minSdk-clang"
        val cxx = "$toolchainBin/aarch64-linux-android$minSdk-clang++"
        val ar = "$toolchainBin/llvm-ar"
        // Plain ProcessBuilder, not Gradle's exec DSL block — that's a
        // Project extension function not reliably reachable from inside a
        // registered task's doLast across Gradle/Kotlin-DSL versions
        // (confirmed: "Unresolved reference 'exec'" against this project's
        // Gradle 9.4 — plain JVM process APIs sidestep that entirely).
        // Dev hot reload (Tier 1): `RSC_HOT=1` (set by `rsc dev --target
        // android`) builds a debug lib WITH the `rosace-ffi/rsc-hot` feature so
        // the app opens its reload socket; otherwise a normal release lib.
        val cargoArgs = mutableListOf("cargo", "build", "--lib", "--target", rustTriple)
        if (System.getenv("RSC_HOT") == "1") {
            cargoArgs.add("--features"); cargoArgs.add("rosace-ffi/rsc-hot")
        } else {
            cargoArgs.add("--release")
        }
        val processBuilder = ProcessBuilder(cargoArgs)
        processBuilder.directory(rootProject.projectDir.parentFile)
        val env = processBuilder.environment()
        env["CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER"] = linker
        env["CC_aarch64-linux-android"] = cc
        env["CXX_aarch64-linux-android"] = cxx
        env["AR_aarch64-linux-android"] = ar
        // `cc`/`cmake` crates and NDK tooling also consult these.
        env["ANDROID_NDK_ROOT"] = ndkHome
        env["PATH"] = "$toolchainBin${File.pathSeparator}${env["PATH"] ?: ""}"
        processBuilder.inheritIO()
        val exitCode = processBuilder.start().waitFor()
        if (exitCode != 0) {
            throw GradleException("cargo build failed with exit code $exitCode")
        }
        val src = rootProject.projectDir.parentFile
            .resolve("target/$rustTriple/release/libgallery_all.so")
        val destDir = projectDir.resolve("src/main/jniLibs/$abi")
        destDir.mkdirs()
        src.copyTo(destDir.resolve("libgallery_all.so"), overwrite = true)
    }
}

tasks.named("preBuild") {
    dependsOn("cargoBuildAndroid")
}

dependencies {
}
