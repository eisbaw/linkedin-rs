# LIX Feature Flag System (LinkedIn Internal eXperiments)

Reverse-engineered from `com.linkedin.android.lixclient` package. Analysis date: 2026-03-24.

---

## 1. Overview

LIX (LinkedIn Internal eXperiments) is LinkedIn's feature flag / A/B testing system. The Android client fetches "treatments" (flag values) from a dedicated LIX frontend service, caches them locally, and polls for updates on a periodic schedule. Every feature gate in the app resolves to a string treatment value (e.g. `"control"`, `"enabled"`, `"v2"`, `"delay-4hrs"`).

Two independent `LixManager` instances exist at runtime:

| Manager | `lixType` | Identity context | Syncs when | Persistent cache |
|---------|-----------|------------------|------------|------------------|
| `AuthenticatedLixManagerImpl` | `1` | `AUTH` | On login + periodic | Yes (disk) |
| `GuestLixManagerImpl` | `0` | `GUEST` | Immediately on start | Yes (disk) |

---

## 2. Endpoint URL and HTTP Method

### V2 (current default)

```
POST {baseUrl}/lix/lixFrontendTreatmentsV2?action=batchGet
```

### V3 (gated by `useV3Endpoint` constructor flag)

```
POST {baseUrl}/lix/lixFrontendTreatmentsV3?action=batchGet
```

The `useV3Endpoint` boolean is passed into `LixNetworkManager` at construction time. Both `AuthenticatedLixManagerImpl` and `GuestLixManagerImpl` currently pass `false` (V2).

Note: This is a Rest.li **action** endpoint (`?action=batchGet`), not a standard CRUD resource. The HTTP method is `POST`.

Source: `LixNetworkManager.getLixEndpoint()`, `AuthenticatedLixManagerImpl` constructor, `GuestLixManagerImpl` constructor.

---

## 3. Request Format

### 3.1 HTTP Headers

| Header | Value | Notes |
|--------|-------|-------|
| `x-restli-protocol-version` | `2.0.0` | Standard Rest.li header |
| `x-restli-method` | `action` | Indicates this is a Rest.li action call |
| `Accept` | `application/vnd.linkedin.deduped+x-protobuf` | Default; falls back to `application/json` if `forceHierarchicalJson` is set |
| `Content-Type` | `application/x-protobuf2 ;symbol-table=lixfrontend-{N}` | Custom protobuf with symbol table; `{N}` = symbol table size. Falls back to `application/json` |
| `x-restli-symbol-table-name` | `lixfrontend-{N}` | References the protobuf symbol table |
| `X-li-page-instance` | `{page instance header}` | Background page instance for tracking |

The default wire format is **protobuf** (a custom LinkedIn protobuf variant called `x-protobuf2` with a symbol table for field name compression). JSON is the fallback when `forceHierarchicalJson` is true.

Source: `LixNetworkManager.buildPostRequest()`.

### 3.2 Request Body (V2 -- `LixBatchGetContext`)

The POST body is a JSON/protobuf-encoded `LixBatchGetContext` record wrapped in a `context` field:

```json
{
  "context": {
    "keys": ["zephyr.client.staff", "voyager.infra.android.launch-rate-limiter", ...],
    "targetIdentityContext": "AUTH" | "GUEST",
    "evaluationContext": { ... },
    "explicitUrnBasedLixes": [
      {
        "urn": "urn:li:member:12345",
        "testKeys": ["some.lix.key"]
      }
    ]
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `keys` | `List<String>` | Yes | LIX test key names to evaluate (excludes URN-based ones) |
| `targetIdentityContext` | Enum: `AUTH`, `GUEST` | No | Whether user is authenticated or guest |
| `evaluationContext` | `EvaluationContextModel` | No | Custom evaluation context for targeting |
| `explicitUrnBasedLixes` | `List<ExplicitUrnBasedLiX>` | No | LIX keys that require a specific URN for evaluation |

Each `ExplicitUrnBasedLiX` contains:
- `urn`: The entity URN to evaluate against (e.g. a member URN)
- `testKeys`: List of LIX key names to evaluate against that URN

The `keys` list contains only LIX definitions that are NOT in `urnBasedLixMap`. URN-based LIX flags are sent separately in `explicitUrnBasedLixes`.

Source: `LixV2BatchGetFactory.createBatchGetContext()`, `LixBatchGetContext`.

### 3.3 Request Body (V3 -- `LixBatchGetContextV3`)

```json
{
  "context": {
    "lixTestkeys": ["zephyr.client.staff", ...],
    "namedUrnsArray": [
      {
        "namedUrns": { "someKey": "urn:li:member:12345" },
        "namedUrnsProviders": {
          "default": {
            "code": "MEMBER_IDENTITY_TOKEN_PRIMARY" | "BCOOKIE_BROWSER_ID"
          }
        }
      }
    ],
    "evaluationContext": {
      "context": { ... }
    }
  }
}
```

V3 differences from V2:
- `keys` renamed to `lixTestkeys` (all keys, no exclusion)
- `targetIdentityContext` replaced by `namedUrnsProviders` with explicit identity resolution codes
- `namedUrnsArray` contains named URN mappings with provider information
- Identity provider codes: `MEMBER_IDENTITY_TOKEN_PRIMARY` (auth, lixType 1 or 3), `BCOOKIE_BROWSER_ID` (guest, lixType 0)

Source: `LixV3BatchGetFactory.createBatchGetContext()`, `LixBatchGetContextV3`.

---

## 4. Response Format

### 4.1 V2 Response (`LixBatchGetResponse`)

The V2 response is parsed as `LixBatchGetResponse` which contains a `value` field that is a `List<LixTreatment>`.

### 4.2 V3 Response (`LixBatchGetResponseV3`)

The V3 response is wrapped in an `ActionResponse` envelope. The inner `LixBatchGetResponseV3` contains a `value` field that is `List<List<LixTreatment>>` -- the first list element is used.

### 4.3 `LixTreatment` Model

Each treatment in the response:

```json
{
  "testKey": "zephyr.client.staff",
  "treatment": "enabled",
  "primaryEvaluationUrn": "urn:li:member:12345",
  "trackingInfo": {
    "experimentId": 12345,
    "treatmentIndex": 0,
    "segmentIndex": 1,
    "urn": "urn:li:...",
    "trackingUrns": ["urn:li:...", ...]
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `testKey` | `String` | Yes | The LIX key name (e.g. `"zephyr.client.staff"`) |
| `treatment` | `String` | Yes | The treatment value (e.g. `"control"`, `"enabled"`, `"v2"`) |
| `primaryEvaluationUrn` | `Urn` | No | The URN used for evaluation |
| `trackingInfo` | `LixTreatmentTrackingInfo` | No | Experiment tracking metadata |

`LixTreatmentTrackingInfo` fields:
- `experimentId` (int) -- server-side experiment identifier
- `treatmentIndex` (int) -- which treatment arm
- `segmentIndex` (int) -- which targeting segment
- `urn` (Urn) -- primary tracking URN
- `trackingUrns` (List<Urn>) -- additional tracking URNs

Treatment values are arbitrary strings. Common patterns observed in the codebase:
- `"control"` -- control group (feature disabled, this is the default for most flags)
- `"enabled"` -- feature enabled
- `"v2"`, `"v3"` -- variant versions
- `"delay-4hrs"` -- behavioral variants
- `"S60"` -- parameterized values (e.g. timeout of 60 seconds)
- Custom strings interpreted per-feature

Source: `LixTreatment`, `LixTreatmentTrackingInfo`, `LixTreatmentsResponseListener.buildLixTreatmentsMap()`.

---

## 5. Caching Mechanism

### 5.1 Two-Tier Cache

The LIX system uses a two-tier cache managed by `LixCacheManager`:

```
LixCacheManager
  -> LixMemoryCache (in-memory HashMap, always present as initial fallback)
  -> LixDiskCache (LMDB key-value store, loaded asynchronously on init)
```

`LixCacheFactory.getCache()` creates either a `LixDiskCache` (when `usePersistentCache=true`) or `LixMemoryCache`. Both `AuthenticatedLixManagerImpl` and `GuestLixManagerImpl` use persistent (disk) caching.

### 5.2 LixMemoryCache

- Stores `Map<LixDefinition, LixTreatment>` in a `Collections.unmodifiableMap`
- On `purgeAndSave()`: replaces the entire map atomically
- On `clear()`: sets map to `Collections.emptyMap()`
- No TTL or size limits

### 5.3 LixDiskCache

- Uses **LMDB** (Lightning Memory-Mapped Database) via `com.linkedin.android.lmdb`
- Storage path: `{appFilesDir}/lixlib-{lixType}` (e.g. `lixlib-0` for guest, `lixlib-1` for auth)
- Map size: 10 MB (`setMapSize(10485760L)`)
- Data format: LinkedIn's custom protobuf via `FissionProtobufDataWriterFactory` / `FissionProtobufDataReaderFactory` with symbol table compression
- Keys are the LIX test key name strings (e.g. `"zephyr.client.staff"`)
- Values are serialized `LixTreatment` records
- Writes happen on a dedicated single-thread executor (`LixDiskCache-writer`)
- On `purgeAndSave()`: drops all existing data, then writes the new batch in a single transaction
- Read transactions are read-only LMDB transactions

### 5.4 Cache Lifecycle

1. **Init**: `LixCacheManager` submits async init to create the disk cache. Meanwhile, a `LixMemoryCache` is used as fallback.
2. **Boot**: On construction, `LixManagerImpl` loads all cached treatments from disk into the in-memory `treatmentsMap` on a background thread, then posts the result to main thread.
3. **Sync**: After network fetch, `purgeAndSave()` replaces the entire cache content with the new server response.
4. **Logout**: `removeCache()` clears the cache, resets sync time, and clears the in-memory treatments map.

### 5.5 Sync Schedule

- **Periodic sync**: Every 15 minutes (`SYNC_INTERVAL = TimeUnit.MINUTES.toMillis(15)`)
- **Sync interval offset**: 5 second grace period (`SYNC_INTERVAL_OFFSET = TimeUnit.SECONDS.toMillis(5)`) -- if less than 14:55 has elapsed since last sync, skip it
- **App foreground sync**: When the app returns to foreground, if time since last background/sync exceeds `maxBackgroundTime`, triggers a forced sync
- **Login sync**: `AuthenticatedLixManagerImpl.onLogin()` triggers an immediate sync
- **Guest sync**: `GuestLixManagerImpl.start()` triggers an immediate sync

Sync timestamps are persisted in SharedPreferences under key `"LixManagerSync-{lixType}"` -> `"LastSyncTime"`.

Source: `LixCacheManager`, `LixDiskCache`, `LixMemoryCache`, `LixCacheFactory`, `LixManagerImpl`.

---

## 6. Override Mechanism

### 6.1 Cookie-Based Override (`lror` cookie)

LIX overrides are stored as a cookie named `lror` (likely "LIX Remote Override"). The `LixOverrideCookieHelper` class manages this cookie.

**Cookie format**:
```
lror=key1=value1&key2=value2&...
```

Where:
- Key-value pairs are delimited by `=` (Huawei's `ContainerUtils.KEY_VALUE_DELIMITER`)
- Pairs are delimited by `&` (Huawei's `ContainerUtils.FIELD_DELIMITER`)
- Cookie expiry: 10 days (hardcoded in `LixOverrideCookieHelper.overwrite()` via `createHttpCookie(uri, "lror", value, 10, false)`)

### 6.2 Override Flow

1. `LixOverrideManager.handleOverride(key, value)` is called (e.g. from dev settings UI)
2. If `value` is non-null: saves the override to the `lror` cookie via `LinkedInHttpCookieManager.saveLixOverride()`
3. If `value` is null: removes that key from the `lror` cookie
4. Applies the override locally to the in-memory `clientOverridesMap` via `LixManagerImpl.applyLocalOverrideIfPossible()`
5. Triggers a re-sync to update from server

### 6.3 Override Precedence

In `LixManagerImpl.getTreatment()`:

```
1. Check clientOverridesMap (local overrides from lror cookie) -- highest priority
2. Check treatmentsMap (from server response / cache)
3. Fall back to LixDefinition.getDefaultTreatment() -- lowest priority (usually "control")
```

### 6.4 Override Persistence

On construction, `LixManagerImpl.loadExistingOverridesFromLrorCookies()` reads the `lror` cookie and populates the `clientOverridesMap`. This means overrides survive app restarts as long as the cookie hasn't expired (10 days).

The `lror` cookie is sent to the server with subsequent requests, so the server is also aware of the overrides. This is the mechanism by which LinkedIn engineers can override feature flags during testing -- the cookie travels with all HTTP requests to `www.linkedin.com`.

Source: `LixOverrideManager`, `LixOverrideCookieHelper`, `LixManagerImpl.getTreatment()`, `LixManagerImpl.loadExistingOverridesFromLrorCookies()`.

---

## 7. How Treatments Are Consumed in App Code

### 7.1 LixDefinition Enums

Feature flags are declared as enum constants implementing `LixDefinition`:

```java
// Authenticated user flags (lixType=1)
public enum Lix implements LixDefinition {
    STAFF("zephyr.client.staff"),
    INFRA_APP_LAUNCH_RATE_LIMITER("voyager.infra.android.launch-rate-limiter", "S60"),
    GROWTH_LAUNCHPAD("zephyr.growth.android.launchpad"),
    // ... ~100+ more
}

// Guest/pre-auth flags (lixType=0)
public enum GuestLix implements LixDefinition {
    MANDATORY_PHONE_REG("zephyr.android.wwe.phoneOnlyRegistration", "enabled"),
    L2M_LOGIN_V2("zephyr.l2m.client.login-v2"),
    // ...
}
```

Each enum value has:
- `name`: The server-side test key string (e.g. `"zephyr.client.staff"`)
- `defaultTreatment`: Fallback value when server hasn't responded (defaults to `"control"`)

### 7.2 Consumption Pattern

Feature code calls `lixManager.getTreatment(LixDefinition)` and compares the returned string:

```java
// Boolean gate
if ("enabled".equals(lixManager.getTreatment(Lix.SOME_FEATURE))) {
    // feature is on
}

// Multi-variant
String treatment = lixManager.getTreatment(GuestLix.SOME_FEATURE);
if ("v2".equals(treatment)) {
    // variant 2
} else if ("v3".equals(treatment)) {
    // variant 3
} else {
    // control
}

// Parameterized
joinModel.phoneOnlyLixOn = "enabled".equalsIgnoreCase(
    guestLixManager.getTreatment(GuestLix.MANDATORY_PHONE_REG));
```

### 7.3 Treatment Change Listeners

Components can subscribe to treatment changes:

```java
lixManager.addTreatmentListener(Lix.SOME_FEATURE, newTreatment -> {
    // react to live treatment change
});
```

Changes are detected by comparing the old cached treatment with the new server response. If the `treatment` string or `trackingInfo` differs, listeners are notified on the main thread.

A `LocalBroadcast` with action `"LixManagerTreatmentsUpdated"` is also sent after every treatment update.

### 7.4 Tracking

Each treatment retrieval (when `getTreatment()` is called) fires a tracking event via `LixNetworkManager.trackLixTreatment()`. This sends a `LixTreatmentsEvent` containing the experiment's tracking metadata. Rate-limited to once per minute per `testKey:lixType` combination.

Source: `Lix.java`, `GuestLix.java`, `LixManager.getTreatment()`, `LixManagerImpl.handleLixUpdate()`.

---

## 8. LIX Key Naming Conventions

From the enum declarations, test keys follow a hierarchical naming pattern:

```
{product}.{domain}.{platform}.{feature-name}
```

| Prefix | Meaning |
|--------|---------|
| `zephyr.*` | China/Zephyr variant features |
| `voyager.*` | International/Voyager variant features |
| `registration.frontend.web.*` | Registration features (shared with web) |

Platform suffixes: `android`, `client` (cross-platform), `web`

---

## 9. Summary for API Replication

To fetch LIX treatments:

```
POST https://www.linkedin.com/lix/lixFrontendTreatmentsV2?action=batchGet
Content-Type: application/json
x-restli-protocol-version: 2.0.0
x-restli-method: action
Accept: application/json

{
  "context": {
    "keys": ["zephyr.client.staff", "voyager.infra.android.launch-rate-limiter"],
    "targetIdentityContext": "AUTH"
  }
}
```

Expected response: a list of `LixTreatment` objects with `testKey` and `treatment` string fields.

Key considerations:
- This endpoint is under `/lix/` prefix, NOT under `/voyager/api/` or `/zephyr/api/`
- Requires valid session cookies (`li_at`, `JSESSIONID`) for authenticated requests
- Guest requests use `targetIdentityContext: "GUEST"` and likely only need `JSESSIONID`
- The override cookie `lror` (if present) will cause the server to return overridden treatment values
- Default wire format is protobuf with symbol tables; use `application/json` for simpler integration
