# International (Voyager) vs Zephyr (China) Build Diff

Comparison of decompiled LinkedIn Android APKs:
- **Zephyr** (China variant): `decompiled/jadx/sources/`
- **International** (Voyager): `decompiled/jadx_intl/sources/`

Analysis date: 2026-03-24.

---

## 1. Routes.java: API Prefix and Endpoint Differences

### API Prefix (Critical Difference)

| Build | API Root | Code Location |
|-------|----------|---------------|
| **Zephyr** | `/zephyr/api/` | `STR_ROOT = "/zephyr/api/"`, `STR_ROOT_ZEPHYR = "/zephyr/api/"` -- hardcoded static fields |
| **International** | `/voyager/api/` | Hardcoded directly in `buildUponRoot()`: `new Uri.Builder().path("/voyager/api/")...` |

The Zephyr build has two root builder methods (`buildUponRoot()` uses `STR_ROOT`, `buildUponZephyrRoot()` uses `STR_ROOT_ZEPHYR`), both pointing to `/zephyr/api/`. The international build has only `buildUponRoot()` with `/voyager/api/` baked in -- no static field indirection, no separate Zephyr root method.

**Rust client recommendation**: Use `/voyager/api/` as the API prefix. The `/zephyr/api/` prefix is exclusively for the China build.

### Route Enum Counts

| Build | Enum Entries | Lines |
|-------|-------------|-------|
| Zephyr | ~376 | 671 |
| International | ~386 | 459 |

Despite having more route entries, the international build is shorter because it lacks the PatchProxy boilerplate throughout the utility methods.

### Zephyr-Only Routes (China-Specific Features)

The Zephyr build has ~126 lines referencing "zephyr" -- routes for China-specific features absent from the international build:

| Category | Example Routes |
|----------|---------------|
| **Nearby People** | `zephyrNearbyV2`, `zephyrNearbyPeopleEntryStatus` |
| **QQ/WeChat integration** | `zephyrGrowthWeChatInviteInfo`, `growth/qqABIConfig` |
| **Career coaching** | `zephyrCoachCampaign*` (6+ routes) |
| **Company reviews** | `zephyrReviewedCompanies`, `zephyrReviewedTags` |
| **Career insights** | `careerInsights`, `zephyrCareerInsightCompanies` |
| **Learning** | `zephyrLearningFeaturedCourses`, `zephyrLearningMiniCourses` |
| **Rewards/Coupons** | `zephyrGrowthMiniCoupons`, `zephyrMissions` |
| **Salary** | `zephyrSalaryInsights`, `zephyrSalarySubmissionStatus` |
| **Flash login** | Flash ID carrier-based auth (Shanyan SDK) |
| **Mars campaigns** | `zephyrMarsCampaign` |
| **Hashtag system** | `zephyrHashtags`, `zephyrHashtagBox` |
| **Polls** | `zephyrNormPolls`, `zephyrPollVotes` |
| **Hotfix patching** | `zephyrDefaultAndroidHotFixPatch` |
| **Barcode/Privacy** | `zephyrBarcodePrivacySettings` |
| **Scholarship** | `zephyrMiniProgramScholarshipCampaign*` |

### International-Only Routes ("Dash" Generation)

The international build has ~219 references to "Dash" -- a newer API generation absent from Zephyr:

| Category | Example Routes |
|----------|---------------|
| **GraphQL** | `GRAPHQL("graphql")` -- entirely absent from Zephyr |
| **Profile Dash** | `voyagerIdentityDashProfiles`, `voyagerIdentityDashProfilePositions`, `voyagerIdentityDashProfileSkills`, etc. |
| **Feed Dash** | `voyagerFeedDashReposts`, `voyagerSocialDashNormComments`, `voyagerSocialDashReactions` |
| **Jobs Dash** | `voyagerJobsDashJobCards`, `voyagerJobsDashJobPostings`, `voyagerJobsDashJobSeekerPreferences` |
| **Notifications Dash** | `notifications/dash/cards`, `voyagerNotificationsDashBadge` |
| **Messaging Dash** | `voyagerMessagingDashMessagingBadge`, `voyagerMessagingDashSalesMessaging` |
| **Relationships Dash** | `voyagerRelationshipsDashMemberRelationships`, `voyagerRelationshipsDashInvitations` |
| **Groups Dash** | `voyagerGroupsDashGroups`, `voyagerGroupsDashGroupMemberships` |
| **Premium Dash** | `voyagerPremiumDashPremiumChooserFlow`, `voyagerPremiumDashAnalyticsCard` |
| **Content Dash** | `voyagerContentcreationDashShares`, `voyagerContentcreationDashWritingAssistant` |
| **Feed V2** | `feed/updatesV2`, `feed/updateV2Actions` |
| **Checkpoint** | `checkpoint/challengesV3`, `checkpoint/login/tokenExchange/upgrade` |

### Shared Core Routes (Present in Both)

These fundamental routes exist in both builds with identical paths:

- `ME("me")`, `FEED("feed/updates")`, `FEED_COMMENTS("feed/comments")`
- `PROFILE("identity/profiles")`, `MINIPROFILE("identity/miniprofiles")`
- `MESSAGING_CONVERSATIONS("messaging/conversations")`
- `JOB_POSTINGS("jobs/jobPostings")`, `JOB_SEARCH("jobs/search")`
- `SEARCH("search/hits")`, `TYPEAHEAD("typeahead/hits")` (Zephyr) vs removed (intl)
- `MUX("mux")`, `SECURE_MUX("mux/secure")`
- `LIX("lixTreatments")`
- `CONTENT_CREATION("contentcreation/normShares")`

**Rust client recommendation**: The "Dash" routes are the newer API surface. For a Rust client targeting the international build, prefer `voyager*Dash*` route variants when available. The non-Dash equivalents still work but may be deprecated. The `graphql` route is international-only and suggests LinkedIn is migrating toward GraphQL.

---

## 2. Auth Flow Comparison

### Core Auth Mechanism: Identical

Both builds use the same cookie-based auth:
- `POST /uas/authenticate` with form-encoded credentials
- CSRF via `JSESSIONID` cookie echoed as `Csrf-Token` header
- Session cookie: `li_at`
- Same JSESSIONID generation: `"ajax:" + 19-digit zero-padded SecureRandom`

### CSRF Generation: Functionally Identical

| Aspect | Zephyr | International |
|--------|--------|---------------|
| Algorithm | `SecureRandom.nextLong()`, `Math.abs()`, `%019d` format | Identical |
| Cookie name | `JSESSIONID` | Identical |
| Cookie format | `ajax:{19-digit}` | Identical |
| Max-age | 100 | 100 |
| Secure flag | true | true |

Code-level differences are cosmetic:
- Zephyr: `CsrfCookieHelper` uses `HttpCookieManager` interface
- International: `CsrfCookieHelper` uses `LinkedInHttpCookieManager` directly
- International: No PatchProxy boilerplate (cleaner code)

### CSRF Token Bootstrap: Same Endpoint

Both call `GET {baseHost}/uas/authenticate` if no JSESSIONID cookie exists. The international build uses `DnsOptions` as the listener class (likely a decompilation rename), while Zephyr uses an anonymous `HttpOperationListener`.

### Passkey Support: International Only

The international build has passkey/WebAuthn support:
- `PasskeyInitialResponse.java` -- FIDO2 passkey handshake response parser
- `LiPasskeyResponseWrapper.java` -- passkey response wrapper
- `SoogleLoginRequestType.java` -- additional login request type enum
- `LoginHelper$$ExternalSyntheticLambda0.java` -- passkey flow handlers

The Zephyr build has NO passkey classes. This is a significant difference: international users can use passwordless FIDO2 login.

### Flash ID Login: Shared Code, Different Usage

Both builds contain `flashIdToken` / `flashAuthInfoId` handling in their auth libraries (it's in `RegistrationInfo`, `LiAuthWebActivity`). However:
- Zephyr: Actively uses Flash ID via Shanyan SDK (`com.chuanglan.shanyan_sdk`)
- International: Has the code paths but the Shanyan SDK is absent; dead code

### Login Result Values: Identical

Both builds handle the same set of `login_result` values including all the Zephyr Flash variants. The international build also processes them (for shared library compatibility) but they would never be returned by the international server.

### Post-Login Challenge Handling: International Has More

The international build has additional post-login challenge infrastructure:
- `PostLoginChallengeErrorType.java`
- `PostLoginChallengeTaskStatus.java`
- `PostLoginChallengeSharedPreference.java`
- `PostLoginChallengeCounterMetric.java`

These are absent from the Zephyr build, suggesting the international build has more sophisticated post-login security challenges.

### LiAuthImpl: Structural Difference

- Zephyr: Full `LiAuthImpl.java` class (~800+ lines) with complete auth flow
- International: `LiAuthImpl.java` main class not found as single file (only inner classes like `LiAuthImpl$3$2.java`, lambda classes). This suggests either the class was split during decompilation or the architecture was refactored.

**Rust client recommendation**: The auth flow is functionally identical between builds. Use the documented `POST /uas/authenticate` flow. Passkeys are not needed for initial implementation. The core sequence (generate JSESSIONID -> POST credentials -> extract li_at) works for both.

---

## 3. Networking Headers Comparison

### Header Construction

Both builds set the same core headers. The code location differs:

| Header | Zephyr Location | International Location |
|--------|----------------|----------------------|
| `X-UDID` | `HeaderUtil.java` | `LinkedInNetwork.java` |
| `X-LI-Track` | `HeaderUtil.java` | `LinkedInNetwork.java` |
| `X-LI-Lang` | `HeaderUtil.java` | `LinkedInNetwork.java` |
| `Csrf-Token` | `LinkedInHttpCookieManager` | `LinkedInHttpCookieManager` (same) |
| `X-RestLi-Protocol-Version` | `NetworkClientConfigurator.java` | Set elsewhere (not in a single configurator) |

### X-LI-Track Header: Identical Fields

Both builds generate the same JSON blob for `X-LI-Track`:

```json
{
  "appId": "com.linkedin.android",
  "carrier": "{Build.BRAND}",
  "clientVersion": "{versionName}",
  "clientMinorVersion": "{versionCode}",
  "deviceType": "android",
  "dpi": "{density bucket}",
  "language": "{locale}",
  "model": "{MANUFACTURER}_{MODEL}",
  "osName": "ANDROID OS/{release}",
  "osVersion": "{release}",
  "timezone": "{UTC offset hours}"
}
```

The field names and values are identical. One implementation difference: Zephyr uses `osVersion` via `b.a.l` (obfuscated constant), international uses the literal `"osVersion"` string. Functionally the same.

### X-LI-User-Agent: Minor Difference

| Aspect | Zephyr | International |
|--------|--------|---------------|
| Format | `LIAuthLibrary:{version} {pkg}:{ver} {mfg}_{model}:android_{release}` | Same format |
| Version param | Hardcoded `"0.0.3"` in code | Passed as parameter, but called with `"0.0.3"` |

### User-Agent Header: Identical

Both use `"ANDROID OS"` as the User-Agent header.

### Default Auth Headers: Identical

```
X-LI-User-Agent: LIAuthLibrary:0.0.3 com.linkedin.android:{ver} ...
User-Agent: ANDROID OS
Accept-Language: {locale with dashes}
```

### X-RestLi-Protocol-Version: Same Version, Different Application Point

Both use `2.0.0`. Zephyr sets it in `NetworkClientConfigurator.configure()`. The international build sets it per-request or via separate modules (`GPBConstants`, `DataUtils`).

**Rust client recommendation**: Use the same headers for both. The header set is functionally identical:
- `X-RestLi-Protocol-Version: 2.0.0`
- `Csrf-Token: {JSESSIONID value}`
- `X-UDID: {device UUID}`
- `X-LI-Track: {JSON blob}`
- `X-LI-Lang: {locale}`
- `Accept-Language: {locale-with-dashes}`
- `User-Agent: ANDROID OS`

---

## 4. Pegasus Models Comparison

### Model Namespace Structure

| Namespace | Zephyr | International | Notes |
|-----------|--------|---------------|-------|
| `voyager/common` | Yes | Yes | Shared types |
| `voyager/identity` | Yes (11 subdirs) | Yes (4 subdirs) | International is trimmed |
| `voyager/messaging` | Yes (6+ subdirs) | Yes (similar) | Different class sets |
| `voyager/feed` | Yes | Yes | Shared |
| `voyager/jobs` | Yes | Yes | Shared |
| `voyager/search` | Yes | Yes | Shared |
| `voyager/news` | Yes | No | Zephyr has news models |
| `voyager/video` | Yes | No | Zephyr has video models |
| `voyager/hiring` | No | Yes | International has hiring |
| `zephyr/*` | Yes (20+ subdirs) | **No** | China-only models |

### MiniProfile: Largely Identical

Both builds have `voyager.identity.shared.MiniProfile` with the same structure. The international build adds `MiniProfileWithRingStatus` (for creator mode ring indicators).

The Zephyr build has many more `identity/shared` types (highlights, network highlights, etc.) reflecting China-specific features.

### Messaging Models: Different Generation

| Zephyr | International |
|--------|---------------|
| `Conversation.java`, `ConversationBuilder.java` (full model + builder) | `ConversationAccessCode.java`, `ConversationAccessCodeBuilder.java` |
| `Event.java`, `EventBuilder.java` | No standalone Event model |
| `Credits.java`, `CreditsBuilder.java` | No Credits model |
| Full Pegasus-generated models with builders | Leaner set -- heavier reliance on Dash endpoints |

The international build has fewer Pegasus models overall because many entity types have migrated to "Dash" endpoints that likely return a different serialization format (possibly GraphQL-compatible).

### "Dash" Models: International Only

The international Routes.java has 219 "Dash" references. These correspond to a newer API surface where:
- Route paths use `voyager{Domain}Dash{Entity}` naming
- Models are likely served via a different serialization than traditional Pegasus
- `DashGraphQLCompat.java` bridges between Dash entities and legacy Pegasus models

The Zephyr build has only 2 "Dash" references (minimal: `CONNECTIONS_DASH`).

### Zephyr-Only Pegasus Namespace

The `com.linkedin.android.pegasus.gen.zephyr` package (Zephyr-only) contains models for:
- `zephyr.auth` -- Flash auth models
- `zephyr.campaign` -- Campaign/Mars models
- `zephyr.careerpath`, `zephyr.careerinsight` -- Career tools
- `zephyr.coach` -- Coaching/mentorship
- `zephyr.jobs` -- China-specific job features
- `zephyr.learning` -- Learning content
- `zephyr.mars` -- Marketing campaigns
- `zephyr.content` -- China content types
- 20+ subdirectories total

**Rust client recommendation**: For the international build, use the "legacy" Pegasus models (`voyager.identity.shared.MiniProfile`, `voyager.messaging.Conversation`, etc.) initially. These are shared between builds and are the foundation. The Dash models are a newer layer -- only needed for features that exclusively use Dash endpoints. Ignore all `zephyr.*` models.

---

## 5. Base URLs

### Hardcoded Production URLs

| URL | Zephyr | International | Purpose |
|-----|--------|---------------|---------|
| `https://www.linkedin.com` | Default base URL | Default base URL | API + web |
| `https://www.linkedin-ei.com` | EI/staging | EI/staging | Engineering environment |
| `slideshare.www.linkedin.com/upload` | Media upload | Not found | Media upload |

Both builds use `FlagshipSharedPreferences` with the same keys:
- `baseUrl` defaults to `https://www.linkedin.com`
- `authUrl` defaults to `https://www.linkedin.com`

The Zephyr build adds `isUserAMonkey()` check to fallback to EI URL during testing.

### No China-Specific Domain

There is no `linkedin.cn` or China-specific domain in either build. The China build uses the same `www.linkedin.com` domain with a different API path prefix (`/zephyr/api/` vs `/voyager/api/`).

**Rust client recommendation**: Use `https://www.linkedin.com` as the base URL. There is no separate API subdomain. The only difference is the path prefix.

---

## 6. CronetNetworkEngine: Structurally Identical

Both builds use Chromium's Cronet as the HTTP transport:
- Same class hierarchy: `CronetNetworkEngine extends CronetNetworkEngineWithoutExecution implements NetworkEngine`
- Same threading model (ThreadPoolExecutor for init, SynchronousQueue)
- Same cookie handling via `LinkedInHttpCookieManager`
- International adds `CronetNetworkEngineWithoutExecution$$ExternalSyntheticLambda0` (lambda extraction)

The TLS fingerprint behavior is determined by the bundled Cronet library version, not the wrapping Java code.

**Rust client recommendation**: For TLS fingerprinting, what matters is matching Cronet's TLS signature, not the Java wrapper. Use a Rust HTTP client that can mimic Chrome/Cronet TLS fingerprints (e.g., `reqwest` with `rustls` configured to match, or use `boring` with appropriate settings).

---

## 7. Summary: Key Differences for Rust Client

### What's the Same (Use for Rust Client)

1. **Base URL**: `https://www.linkedin.com` (both builds)
2. **Auth endpoint**: `POST /uas/authenticate` (both builds, identical flow)
3. **CSRF generation**: `ajax:{19-digit random}` (identical algorithm)
4. **Session cookie**: `li_at` (both builds)
5. **Required headers**: Identical set (`X-RestLi-Protocol-Version`, `Csrf-Token`, `X-UDID`, `X-LI-Track`, `X-LI-Lang`)
6. **X-LI-Track format**: Same JSON fields and values
7. **Rest.li protocol**: Version 2.0.0 (both builds)
8. **Cookie management**: Same `JSESSIONID` + `li_at` pattern
9. **Core routes**: Feed, profiles, messaging, jobs, search -- same paths

### What's Different (Implications for Rust Client)

| Difference | Impact | Action |
|-----------|--------|--------|
| API prefix: `/voyager/api/` vs `/zephyr/api/` | **High** | Use `/voyager/api/` for international |
| Dash endpoints (intl only) | **Medium** | Prefer Dash variants when available |
| GraphQL route (intl only) | **Low** | Future capability; not needed now |
| Passkey auth (intl only) | **Low** | Not needed for password-based auth |
| Flash ID auth (Zephyr only) | **None** | Irrelevant for international |
| Zephyr-specific routes (~100) | **None** | Irrelevant for international |
| Post-login challenges (intl more) | **Low** | Handle when encountered |
| PatchProxy boilerplate (Zephyr only) | **None** | Cosmetic decompilation difference |

### Recommended Approach

Build the Rust client targeting the **international (Voyager) build**:

1. Use `/voyager/api/` prefix for all API calls
2. Implement the standard `POST /uas/authenticate` flow (identical between builds)
3. Use the same headers as documented (both builds use them identically)
4. Start with legacy Pegasus route paths (`messaging/conversations`, `identity/profiles`, etc.)
5. Migrate to Dash endpoints as needed (they coexist with legacy routes)
6. Ignore all `zephyr*` routes and models entirely
