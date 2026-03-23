# LinkedIn APK File Analysis

Extracted from `com.linkedin.android` APK. Analysis date: 2026-03-23.

## Summary

| Metric | Value |
|--------|-------|
| Total files | 10,493 |
| Total size (extracted) | 145 MB |
| DEX files | 5 (51.6 MB total) |
| Native libraries (.so) | 146 (73 per architecture) |
| Architectures | arm64-v8a, armeabi-v7a |
| Resource files (r/) | 10,220 files, 48 MB |
| Assets | 29 files, 2.7 MB |

## Framework Identification

**Hybrid: Java/Kotlin + React Native (with Hermes engine)**

Evidence:
- 5 DEX files (51.6 MB) indicate substantial Java/Kotlin codebase
- `kotlin/` directory with `.kotlin_builtins` files confirms Kotlin usage
- `assets/index.android.bundle` (2.0 MB) -- React Native JavaScript bundle (Metro bundler, production mode)
- 42 React Native native libraries (libreactnativejni.so, libfabricjni.so, librrc_*.so, etc.)
- `libhermes.so` -- Facebook's Hermes JS engine (used instead of JSC for React Native)
- `libyoga.so` -- Flexbox layout engine used by React Native
- `libreanimated.so` -- React Native Reanimated (animation library)
- React Native Fabric renderer present (libreact_render_*.so, libfabricjni.so)
- TurboModules present (libturbomodulejsijni.so) -- new architecture

The app is primarily a **native Android app written in Java/Kotlin**, with **React Native embedded for select UI surfaces**. This is a common pattern at LinkedIn -- the core app is native, and newer features are built with React Native.

## File Type Inventory

| Extension | Count | Size | Notes |
|-----------|-------|------|-------|
| .webp | 5,941 | 10.3 MB | UI images, icons |
| .xml | 3,741 | 6.1 MB | Android layouts, drawables, configs |
| .png | 539 | 1.0 MB | Legacy images |
| .so | 146 | 41.1 MB | Native libraries (2 architectures) |
| .properties | 62 | 100 KB | SDK version metadata |
| .json | 14 | 218 KB | Lottie animations, config |
| .dex | 5 | 51.6 MB | Dalvik bytecode |
| .kotlin_builtins | 7 | -- | Kotlin stdlib metadata |
| .version | 6 | -- | SDK version markers |
| .html | 4 | 58 KB | Embedded web views |
| .bks | 4 | 64 KB | BouncyCastle keystores (cert pinning) |
| .gif | 3 | 430 KB | Animated images |
| .yaml | 2 | 356 KB | User-agent parser regexes |
| .txt | 2 | 125 KB | Pinyin data, multidex version |
| .bundle | 1 | 2.0 MB | React Native JS bundle |
| .arsc | 1 | 3.5 MB | Compiled Android resources |
| .dat | 1 | 57 KB | Binary data (zlsioh.dat) |
| .bin | 1 | -- | DebugProbesKt.bin (coroutines) |
| .gz | 1 | -- | Compressed data |
| .js | 1 | -- | Zephyr phone number script |

## Directory Structure

| Directory | Size | Files | Contents |
|-----------|------|-------|----------|
| r/ | 48 MB | 10,220 | Android resources (obfuscated names: a/, a0/, aa/, etc.) |
| lib/ | 40 MB | 146 | Native shared libraries |
| assets/ | 2.7 MB | 29 | JS bundle, animations, keystores, HTML |
| META-INF/ | 1.8 MB | 3 | APK signing (MANIFEST.MF, MYKEY.SF, MYKEY.RSA) |
| ua_parser/ | 180 KB | 1 | User-agent parser regexes |
| com/ | 144 KB | -- | Java package resources |
| kotlin/ | 68 KB | 7 | Kotlin stdlib builtins |
| okhttp3/ | 56 KB | 1 | OkHttp public suffix list |

## DEX Files

| File | Size |
|------|------|
| classes.dex | 9.3 MB |
| classes2.dex | 11.2 MB |
| classes3.dex | 13.4 MB |
| classes4.dex | 12.2 MB |
| classes5.dex | 5.5 MB |
| **Total** | **51.6 MB** |

5 DEX files indicates the app exceeds the 64K method limit multiple times over. Heavy use of multidex. Total bytecode size suggests a very large codebase.

## Native Libraries (lib/)

Two architectures with identical library sets (73 .so files each):

### React Native Core (42 libraries)

- `libreactnativejni.so` -- RN bridge
- `libhermes.so` -- Hermes JS engine
- `libhermes-executor-*.so` -- JS executor (debug + release variants)
- `libhermes-inspector.so` -- Chrome DevTools inspector
- `libfabricjni.so` -- Fabric (new rendering system)
- `libreact_render_*.so` -- 16 renderer modules (core, animations, graphics, text, etc.)
- `librrc_*.so` -- 11 React Native components (view, text, image, scrollview, etc.)
- `libturbomodulejsijni.so` -- TurboModules (new native module system)
- `libreact_codegen_rncore.so` -- Codegen output
- `libyoga.so` -- Flexbox layout engine
- `libreanimated.so` -- React Native Reanimated

### React Native Dependencies (6 libraries)

- `libfb.so`, `libfbjni.so` -- Facebook JNI utilities
- `libfolly_futures.so`, `libfolly_json.so` -- Facebook Folly C++ library
- `libglog.so`, `libglog_init.so` -- Google logging

### Image Processing (4 libraries)

- `libimagepipeline.so` -- Fresco image pipeline
- `libnative-filters.so` -- Image filters
- `libnative-imagetranscoder.so` -- Image transcoding
- `libBitmapUtils.so` -- Bitmap utilities

### Networking (1 library)

- `libcronet.83.0.4103.83.so` -- Chromium network stack (v83)

### Platform/Vendor (7 libraries)

- `libc++_shared.so` -- C++ standard library
- `libanrDetector.so` -- ANR detection
- `libbetter.so` -- Unknown (LinkedIn internal?)
- `libbsdiff.so` -- Binary diff (for incremental updates)
- `libCtaApiLib.so` -- Chinese telecom API (regulatory compliance)
- `libgetuiext3.so` -- Getui push SDK (Chinese push notifications)
- `libLMDBAndroid.so` -- Lightning Memory-Mapped Database
- `libndkCrashReporter.so` -- NDK crash reporting
- `librusage_jni.so` -- Resource usage monitoring
- `libsecsdk.so` -- Security SDK
- `libShanYCore.so` -- ShanYan (Chinese phone number verification)

### Security/Regional (3 libraries)

- `libsecsdk.so` -- Security/anti-tampering SDK
- `libCtaApiLib.so` -- Chinese telecom compliance
- `libShanYCore.so` -- Chinese phone verification (ShanYan)

## Assets

Notable files in `assets/`:

| File | Size | Purpose |
|------|------|---------|
| `index.android.bundle` | 2.0 MB | React Native JS bundle (production, Metro) |
| `zephyr_pinyin.txt` | 125 KB | Chinese input (pinyin) data |
| `zlsioh.dat` | 57 KB | Binary data (purpose unknown) |
| `zephyr_add_phone_number.js` | -- | Phone number flow (embedded JS) |
| `*.html` (4 files) | 58 KB | Embedded webviews (Bing Maps, phone number, article template) |
| `*.json` (14 files) | 218 KB | Lottie animations, config, company blacklist |
| `*.bks` (4 files) | 64 KB | BouncyCastle keystores (TLS cert pinning) |

## SDK/Third-Party Integration

Identified from .properties files and library names:

- **Firebase**: analytics, messaging, core, iid (v15.x era)
- **Google Play Services**: ads, auth, awareness, identity, location, maps, places, safetynet, vision
- **Huawei Mobile Services**: HMS (agconnect, hianalytics, hmsincas/hmsrootcas keystores)
- **Chromium Cronet**: Network stack v83.0.4103.83
- **OkHttp3**: HTTP client (public suffix list present)
- **Kotlin Coroutines**: DebugProbesKt.bin, CoroutineExceptionHandler, MainDispatcherFactory
- **LMDB**: Embedded key-value database
- **Fresco**: Facebook image loading library
- **User-Agent Parser**: ua_parser with regex rules

## Obfuscation

- Resource directories use obfuscated names (a/, a0/, a1/, aa/, etc.) -- R8/ProGuard resource shrinking
- R8/ProGuard is confirmed by the obfuscated `r/` directory structure (51 subdirectories with short names)
- DEX files will contain obfuscated class/method names (requires jadx decompilation to assess severity)
- `resources.arsc` (3.5 MB) contains the resource name mappings

## Key Findings for Reverse Engineering

1. **Hybrid architecture**: The app mixes native Java/Kotlin with React Native. API calls may originate from either layer. The JS bundle (`index.android.bundle`) should be analyzed alongside DEX decompilation.

2. **React Native with New Architecture**: Fabric renderer and TurboModules are present, meaning some native modules use the new RN architecture with C++ JSI bindings rather than the old bridge.

3. **Hermes bytecode**: The JS bundle starts with Metro bundler preamble and uses Hermes. The bundle may contain Hermes bytecode (HBC) rather than plain JS, which would require `hermes-dec` or `hbcdump` to decompile.

4. **Networking via Cronet**: LinkedIn uses Chromium's network stack (Cronet v83), not just OkHttp. This means TLS fingerprinting and HTTP/2 behavior will match Chrome, not standard Android.

5. **Chinese market support**: Significant presence of Chinese SDK integrations (Getui push, ShanYan phone verification, CTA telecom API, Huawei HMS, pinyin input). The APK may be a China-variant or unified global build.

6. **Certificate pinning**: 4 BouncyCastle keystore files in assets suggest TLS certificate pinning is active. This will need to be bypassed for traffic interception.

7. **LMDB for local storage**: LinkedIn uses LMDB (libLMDBAndroid.so) as a local embedded database rather than SQLite alone.

8. **Heavy obfuscation expected**: Resource shrinking is active. Class/method obfuscation in DEX will require mapping reconstruction during decompilation.
