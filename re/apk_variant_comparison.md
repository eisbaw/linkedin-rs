# APK Variant Comparison: Zephyr (China) vs Voyager (International)

Date: 2026-03-24

---

## 1. Acquisition

### Download Method

Both APKs obtained via `apkeep` from APKPure. Huawei App Gallery was attempted but returned no download.

| Variant | Version | Download | File Size | Format |
|---------|---------|----------|-----------|--------|
| Zephyr (China) | 6.1.1 (versionCode 111206000) | `apkeep -a com.linkedin.android -d apk-pure` | 54.9 MB | Single APK |
| Voyager (International) | 4.1.1183 (versionCode 209100) | `apkeep -a com.linkedin.android@4.1.1183 -d apk-pure` | 93.8 MB base + 26 MB splits = 120 MB total | XAPK (split APK bundle) |

### APKPure Observations

- APKPure serves **only one variant at a time** as the "latest" (currently 6.1.1 = Zephyr).
- The international variant (4.1.x series) is available as older versions: `4.1.1110` through `4.1.1183`.
- The version numbering schemes are completely different: Zephyr uses `6.x.x`, Voyager uses `4.1.xxxx`.
- The latest APKPure download (6.1.1) is **byte-for-byte identical** to our existing APK (MD5: `2d42e62fda290f8d8c81d8be630d9513`).

### Version Numbering Note

Despite `4.1.1183` appearing "older" by version string, the versionCode tells a different story:
- Zephyr 6.1.1: versionCode `111206000`
- Voyager 4.1.1183: versionCode `209100`

These are independent version tracks. The Voyager 4.1.x series is actively maintained (latest 4.1.1183 is recent) while Zephyr follows a separate `6.x` scheme.

---

## 2. Confirmed: These Are Different Apps

This is not a regional configuration difference -- these are **fundamentally different builds** sharing the same package name (`com.linkedin.android`).

### 2.1 SDK Targets

| | Zephyr | Voyager |
|--|--------|---------|
| minSdkVersion | 21 (Lollipop) | 28 (Pie) |
| targetSdkVersion | 29 (Android 10) | 35 (Android 15) |
| compileSdkVersion | 29 | (not in manifest) |

The international variant targets much newer Android APIs.

### 2.2 Application Entry Point

Both use `com.linkedin.android.app.FlagshipApplication` as the Application class, but with very different configurations:

| Attribute | Zephyr | Voyager |
|-----------|--------|---------|
| label | `@string/d5` (obfuscated) | `@string/voyager_app_name` |
| theme | `@style/axr` (obfuscated) | `@style/VoyagerAppTheme.Mercado` |
| usesCleartextTraffic | **true** | **false** |
| extractNativeLibs | (default) | true |
| dataExtractionRules | absent | `@xml/data_extraction_rules` |
| enableOnBackInvokedCallback | absent | false |
| localeConfig | absent | `@xml/locales_config` |
| requestLegacyExternalStorage | true | absent |
| roundIcon | absent | `@mipmap/ic_launcher_round` |

**Notable**: Zephyr allows cleartext HTTP traffic; Voyager does not.

### 2.3 Resource Obfuscation

Zephyr has heavily obfuscated resource names (`@string/d5`, `@style/axr`, `@style/ay4`). Voyager uses readable names (`@string/voyager_app_name`, `@style/VoyagerAppTheme.Mercado`). This suggests different build pipelines or different ProGuard/R8 configurations.

---

## 3. API Surface: Zephyr vs Voyager

### 3.1 Zephyr API Paths (from JS bundle and decompiled code)

The Zephyr variant uses `/zephyr/api/` as its primary API prefix:

```
/zephyr/api/zephyrMe
/zephyr/api/zephyrMiniJobs
/zephyr/api/zephyrJobSocialHiringReferrerCards
/zephyr/api/zephyrJobSocialHiringReferrers
/zephyr/api/zephyrJobSocialHiringReferrerAwardTasks
/zephyr/api/voyagerContentcreationUrlPreview
/zephyr/api/zephyrCoachCampaignFilterTypeahead
```

Plus the shared: `/voyager/api/contentcreation/normShares`

### 3.2 Voyager API Paths (expected)

The international variant would use `/voyager/api/` exclusively. No `/zephyr/api/` references found in the international APK's decoded assets.

### 3.3 No RN Bundle in International APK

The international APK (4.1.1183) does **not** contain `index.android.bundle` in its assets. The Zephyr variant's React Native layer (social hiring, mini jobs) appears to be China-market-specific functionality that does not exist in the international variant.

---

## 4. Push Notification Infrastructure

| | Zephyr | Voyager |
|--|--------|---------|
| Firebase/FCM | Yes (legacy `FirebaseInstanceIdService`) | Yes (modern `FirebaseMessagingService`) |
| Huawei Push (HMS) | Yes (HMS SDK, GRS configs) | No |
| Xiaomi Push | Yes (via `supplierconfig.json`) | No |
| Oppo Push | Yes | No |
| Vivo Push | Yes | No |
| Google Play Services | Minimal | Full (`com.google.android.gms`) |

The Zephyr variant uses an older Firebase SDK (`FirebaseInstanceIdService`, deprecated) while the international variant uses the current `FirebaseMessagingService` directly.

---

## 5. Third-Party Integrations

| Integration | Zephyr | Voyager |
|-------------|--------|---------|
| Google Sign-In | No | Yes (`SignInHubActivity`) |
| Google Play Billing | No | Yes (`ProxyBillingActivity`) |
| Facebook | No | Yes (`FacebookActivity`, `CustomTabActivity`) |
| WeChat Mini Programs | Yes (`WeChatMiniProgramActivity`) | No |
| China Unicom (CUCC) | Yes (SIM-based phone verification) | No |
| Samsung Sync | No | Yes (`SamsungSyncConsentActivity`) |
| Credentials API | No | Yes (`androidx.credentials`) |
| Biometric Auth | No | Yes (`DeviceCredentialVerificationActivity`) |
| Liveness Check | No | Yes (`LivenessCheckTestActivity`) |
| Identity Verification | No | Yes (`VerificationAutoCloseActivity`) |

---

## 6. Feature Surfaces by Asset Directories

### Zephyr-only assets
```
career/                          # Career features
cucc/                            # China Unicom integration
grs_sdk_*.json                   # Huawei GRS configs
grs_sp.bks, hmsincas.bks, etc.  # HMS TLS certificates
guest_experience.json            # Chinese-language onboarding content
hianalytics_njjn                 # Huawei analytics
index.android.bundle             # React Native layer (social hiring, jobs)
l2m/                             # L2M features
scholarship_*.png                # WeChat sharing images
socialhiring/                    # Social hiring animations
video/                           # Video features
zephyr_*.js/html/txt             # Zephyr-specific phone number, pinyin
zlsioh.dat                       # Unknown binary
```

### Voyager-only assets
```
assessments/                     # Skills assessments
atwork/                          # LinkedIn at Work features
careers/                         # Career pages
coach/                           # LinkedIn Coach
dexopt/baseline.prof             # ART baseline profile (performance)
difwordlist.txt                  # Dictionary/word list
events/                          # Events feature
feedListSkeletonPreview.pb       # Feed skeleton (protobuf)
game/                            # LinkedIn Games
media/                           # Media handling
media-ingestion-spec.json        # Media upload specs (image/video formats)
news/                            # LinkedIn News
onestepposting/                  # One-step posting
org/                             # Organization pages
premium/                         # LinkedIn Premium features
PublicSuffixDatabase.list        # PSL for URL parsing
search/                          # Search feature
```

### Shared assets
```
entities/                        # Entity animations (checkmarks)
feed/                            # Feed animations (double-tap like)
growth/                          # Growth animations
messaging/                       # Messaging (Bing Maps WebView)
templates/                       # HTML templates
```

---

## 7. Manifest Component Counts

| Component Type | Zephyr | Voyager |
|----------------|--------|---------|
| Activities | 169 | 66 |
| Services | 65 | 166 |
| Receivers | 55 | 43 |
| Providers | 13 | 24 |

Zephyr has far more activities (169 vs 66) but fewer services (65 vs 166). This likely reflects:
- Zephyr: more screen-based UI features registered as Activities
- Voyager: more background processing via Services (presumably Server-Driven UI reduces Activity count while background sync/media processing increases Service count)

---

## 8. Key Implications for Reverse Engineering

### 8.1 Our Existing Analysis is Zephyr-Specific

All decompiled code, API routes, and architecture analysis done so far is from the Zephyr (China) variant. The international Voyager app would have:
- Different API base URLs (`/voyager/api/` instead of `/zephyr/api/`)
- No React Native layer (or a different one)
- Different authentication flows (Google Sign-In, biometrics, no WeChat)
- Different networking configuration (no cleartext traffic allowed)
- More modern Android SDK targets

### 8.2 Shared Package Name, Different Apps

Both ship as `com.linkedin.android` and share the `FlagshipApplication` entry point, but they are built from different source trees (or heavily feature-flagged builds). The resource obfuscation differences suggest different build configurations.

### 8.3 What Would Be Needed for Full Voyager Analysis

To analyze the international variant's API surface, we would need to decompile its base APK (93.8 MB). Since it's an XAPK with split APKs, the base APK contains the DEX files. The international APK has been saved as `linkedin_intl_4.1.1183.apk` in the project root.

### 8.4 APKPure Limitation

APKPure currently serves the Zephyr variant as "latest" for `com.linkedin.android`. The international variant is only available as specific older version numbers (4.1.xxxx). This may change over time. Other sources (Google Play via authenticated `apkeep`, APKMirror manual download) might provide the true latest international version.

---

## 9. Files

| File | Description |
|------|-------------|
| `com.linkedin.android.apk` | Zephyr (China) v6.1.1, 54.9 MB |
| `linkedin_intl_4.1.1183.apk` | Voyager (International) v4.1.1183 base APK, 93.8 MB |
