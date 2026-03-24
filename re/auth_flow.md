# LinkedIn Android App: Authentication Flow

Reverse-engineered from `com.linkedin.android` APK, decompiled via jadx.
Analysis date: 2026-03-24.

Primary source classes:
- `com.linkedin.android.liauthlib.LiAuthImpl` -- core auth implementation
- `com.linkedin.android.liauthlib.LiAuth` -- auth interface
- `com.linkedin.android.liauthlib.network.NetworkUtils` -- CSRF token handling
- `com.linkedin.android.networking.cookies.CsrfCookieHelper` -- JSESSIONID generation
- `com.linkedin.android.app.UnauthorizedStatusCodeHandler` -- 401 handling
- `com.linkedin.android.liauthlib.sso.LiSSOService` -- SSO service
- `com.linkedin.android.developer.OAuthService` -- third-party OAuth token service
- `com.linkedin.android.liauthlib.thirdparty.LiThirdPartyAuthorizer` -- OAuth authorization grant
- `com.linkedin.android.liauthlib.thirdparty.LiThirdPartyWebViewAuthorizeActivity` -- OAuth PKCE WebView
- `com.linkedin.android.infra.network.Auth` -- high-level auth facade

---

## 1. Overview

LinkedIn's mobile auth is **cookie-based**, not token-based. There are no OAuth2 bearer tokens for the first-party app. The critical artifacts are:

| Artifact | Purpose |
|----------|---------|
| `JSESSIONID` cookie | CSRF protection -- value echoed as `Csrf-Token` header |
| `li_at` cookie | Primary session cookie -- presence = authenticated |
| `/uas/authenticate` | Login endpoint (GET for CSRF, POST for credentials) |
| `/uas/issueLoginCookie` | Post-login cookie exchange |
| `/uas/directLogout` | Logout |
| `/mob/sso/you` | Profile fetch + session validation after login |

There is **no token refresh mechanism**. The `li_at` cookie is long-lived. If it expires or is revoked, the app logs the user out and requires re-authentication.

---

## 2. CSRF Token (JSESSIONID) Generation

### 2.1 How It Works

Source: `CsrfCookieHelper.generateJsessionId()` in `com.linkedin.android.networking.cookies`

```
Format: "ajax:{19-digit zero-padded random number}"
Example: "ajax:0123456789012345678"
```

Generation algorithm:
1. `SecureRandom.nextLong()` -- cryptographic random
2. `Math.abs(result)` -- ensure positive (special-case `Long.MIN_VALUE` -> `Long.MAX_VALUE`)
3. Format: `String.format(Locale.US, "%019d", value)` -- zero-padded to exactly 19 digits
4. Prefix with `"ajax:"`

The JSESSIONID cookie is created with:
- Domain: derived from the request URI
- Max-age: 100 (seconds? -- unclear, but short-lived in cookie terms)
- Secure: true

### 2.2 When It's Created

Source: `CsrfCookieHelper.readOrCreateIfNull()`

The JSESSIONID is lazily created. `readOrCreateIfNull()` checks if one exists in the cookie store for the given URI. If not, it generates a new one and saves it.

### 2.3 How It's Used

Every authenticated request includes:
```
Cookie: JSESSIONID=ajax:0123456789012345678
Csrf-Token: ajax:0123456789012345678
```

The `Csrf-Token` header value is read from the JSESSIONID cookie value. The server validates that these match.

### 2.4 CSRF Token Acquisition Before Auth

Source: `NetworkUtils.performRequestWithCSRFToken()`

Before any auth operation, the library checks if a JSESSIONID cookie exists:
1. Read `JSESSIONID` cookie for the target host
2. If present and non-empty: proceed immediately (run the callback)
3. If missing: `GET {baseHost}/uas/authenticate` -- the server responds with a `Set-Cookie: JSESSIONID=...` header
4. If the GET succeeds (2xx): proceed with the original operation
5. If the GET fails: report the error to the CSRF listener

**Key insight for Rust client**: Before calling `POST /uas/authenticate`, you must either:
- Generate a client-side JSESSIONID cookie using the algorithm above, OR
- Make a `GET /uas/authenticate` to receive a server-generated one

The client-side generation approach (used by the networking layer) is simpler and avoids an extra round trip.

---

## 3. Primary Login Flow (Email/Password)

### 3.1 Sequence Diagram

```
Client                                  Server
  |                                        |
  |-- Generate JSESSIONID cookie -------->|  (client-side, or GET /uas/authenticate)
  |                                        |
  |-- POST /uas/authenticate ------------>|
  |   Content-Type: application/x-www-form-urlencoded
  |   Cookie: JSESSIONID=ajax:...
  |   Body: session_key={email}&session_password={pass}
  |          [&rememberMeOptIn=true]
  |          [&client_enabled_features=ANDROID_NATIVE_CAPTCHA]
  |          [&{existing cookies as name=value pairs}]
  |                                        |
  |<-- 200 OK -----------------------------|
  |   Set-Cookie: li_at=...               |
  |   Body: {"login_result": "PASS"}      |
  |                                        |
  |-- POST /uas/issueLoginCookie -------->|  (optional, for additional cookies)
  |   Content-Type: application/x-www-form-urlencoded
  |   Body: cookies={cookie_string}
  |                                        |
  |<-- 200 OK -----------------------------|
  |                                        |
  |-- GET /mob/sso/you ------------------>|  (fetch profile data)
  |                                        |
  |<-- 200 OK -----------------------------|
  |   Body: {"firstName":..., "lastName":..., "memberID":..., ...}
  |                                        |
```

### 3.2 POST /uas/authenticate Request

**URL**: `{baseHost}/uas/authenticate`
**Method**: POST
**Content-Type**: `application/x-www-form-urlencoded`

**Form parameters** (built as `List<Pair<String, String>>`, URL-encoded):

| Parameter | When Included | Value |
|-----------|--------------|-------|
| `session_key` | Email/phone login (no Google/Flash token) | User email or phone number |
| `session_password` | Email/phone login | User password |
| `session_midToken` | Mid-token login (fastrack) | Mid token string |
| `googleIdToken` | Google Sign-In | Google ID token |
| `flashIdToken` | Flash ID login (China) | Flash ID token |
| `flashAuthInfoId` | Flash ID login (China) | Flash auth info ID |
| `appleIdToken` | Apple Sign-In | Apple ID token |
| `appleAuthCode` | Apple Sign-In | Apple authorization code |
| `Challenge_id` | After CAPTCHA challenge | Challenge identifier |
| `client_enabled_features` | If native CAPTCHA LIX enabled | `"ANDROID_NATIVE_CAPTCHA"` |
| `rememberMeOptIn` | If remember-me UI shown | `"true"` or `"false"` |
| `{cookie_name}` | Always -- existing cookies | Cookie values from LinkedIn domains |

**Important**: The request body includes ALL existing cookies for LinkedIn domains as additional form parameters. This is done by `mHttpStack.getCookieNameValuePairs(cookieDomainSpec)`.

Cookie domains scanned:
- `.linkedin.com`, `linkedin.com`, `.www.linkedin.com`, `www.linkedin.com`
- `.linkedin-ei.com`, `linkedin-ei.com`, `.www.linkedin-ei.com`, `www.linkedin-ei.com`
- (With LIX flag: also `https://www.linkedin.com`, `https://www.linkedin-ei.com`)

### 3.3 POST /uas/authenticate Response

**Success (HTTP 200)**:
```json
{"login_result": "PASS"}
```
The `li_at` session cookie is set via `Set-Cookie` response header.

**Challenge (HTTP 401)**:
```json
{
  "login_result": "CHALLENGE",
  "challenge_url": "https://www.linkedin.com/checkpoint/...",
  "Challenge_id": "abc123",
  "session_key": "user@example.com"
}
```

**Other `login_result` values** (all HTTP 401):

| Value | Meaning |
|-------|---------|
| `PASS` | Success |
| `GOOGLE_LOGIN_MID_TOKEN_MISMATCH` | Also treated as success |
| `BAD_EMAIL` | Invalid email/phone |
| `BAD_PASSWORD` | Wrong password |
| `LOGIN_RESTRICTED` | Account restricted |
| `CHALLENGE` | Requires CAPTCHA/2FA |
| `USER_CANCELLED` | User cancelled login |
| `GOOGLE_LOGIN_NO_ACCOUNT` | No LinkedIn account linked to Google |
| `GOOGLE_LOGIN_DENIED` | Google login denied |
| `GOOGLE_TOKEN_UNVERIFIED` | Google token validation failed |
| `APPLE_LOGIN_NO_ACCOUNT` | No LinkedIn account linked to Apple |
| `APPLE_LOGIN_DENIED` | Apple login denied |
| `APPLE_TOKEN_UNVERIFIED` | Apple token validation failed |
| `APPLE_EMAIL_AND_USER_ID_MAPS_TO_DIFFERENT_MEMBER_ID` | Apple account conflict |
| `ZEPHYR_FLASH_LOGIN_NO_ACCOUNT` | Flash login no account (China) |
| `ZEPHYR_FLASH_LOGIN_DENIED` | Flash login denied (China) |

### 3.4 Post-Login Steps

After HTTP 200 from `/uas/authenticate`:

1. **Save username** to `SharedPreferences` key `"auth_username"` in `"auth_library_prefs"`
2. **Fetch profile data**: `GET {baseHost}/mob/sso/you`
   - Returns JSON with: `firstName`, `lastName`, `memberID`, `emailAddress`, `headline`, `pictureUrl`
   - Profile data saved to SharedPreferences: `auth_member_id`, `auth_first_name`, `auth_last_name`, `auth_short_full_name`, `auth_full_name`, `auth_headline`, `auth_picture_url`
3. **Publish user MID**: Broadcasts `"ACTIVE_USER_INTENT_ACTION"` with the member ID
4. **Fetch profile picture**: If `pictureUrl` is present, downloads and caches it locally

### 3.5 POST /uas/issueLoginCookie

Called after challenge-based auth (WebView flow), NOT after direct login.

**URL**: `{baseHost}/uas/issueLoginCookie`
**Method**: POST
**Content-Type**: `application/x-www-form-urlencoded`
**Body**: `cookies={url_encoded_cookie_string}`

This converts WebView cookies into the app's cookie store.

---

## 4. Challenge / CAPTCHA Handling

### 4.1 Challenge Detection

When `/uas/authenticate` returns HTTP 401 with `login_result: "CHALLENGE"`, the app handles it differently based on the presence of `Challenge_id`:

**Path A: WebView Challenge (no Challenge_id)**
1. Open `LiAuthWebActivity` with the `challenge_url`
2. User completes challenge in WebView (CAPTCHA, email verification, etc.)
3. WebView broadcasts result via `com.linkedin.android.liauthlib.intent.webAuthenticationCompleted`
4. On success: call `authenticateAfterChallenge()` -> `POST /uas/issueLoginCookie` with WebView cookies

**Path B: Native CAPTCHA (Challenge_id present)**
1. If `ANDROID_NATIVE_CAPTCHA` feature is enabled:
   - Use Google SafetyNet reCAPTCHA v2
   - Site key: `6LdcXFUUAAAAANu32LPaK4zJiXYrjHj4efRGaMFu`
   - On token received: `POST {baseHost}/checkpoint/challenges/nativeCaptchaChallenge/{challengeId}?displayTime={ns}`
   - Body: `{"userResponseToken": "{recaptcha_token}"}`
   - On SUCCESS response: retry `authenticate()` with original credentials
   - On FAILURE response: retry native CAPTCHA
   - On INTERNAL_ERROR: report error to user

### 4.2 WebView Auth Activity

`LiAuthWebActivity` receives these extras:
- `webview_authentication_url` -- challenge URL to load
- `username`, `password`, `midToken`, `googleIdToken`, `flashIdToken`, `flashAuthInfoId`, `appleIdToken`, `appleAuthCode` -- credentials to inject
- `host` -- base host
- `useragent` -- `LIAuthLibrary:0.0.3 com.linkedin.android:... Manufacturer_Model:android_...`
- `should_use_updated_user_agent` -- true
- `webview_authentication_type` -- `"challenge"`, `"remember_me"`, or `"apple_sign_in"`

The WebView intercepts the result and broadcasts it back. The result includes:
- `webview_authentication_result` -- the login_result string
- `webview_authentication_cookies` -- cookies from the WebView session
- `webview_bridge_result` -- for Remember Me: `"SIGN_IN"`, `"JOIN_NOW"`, etc.
- `webview_bridge_message` -- additional message data

---

## 5. Cookie Lifecycle

### 5.1 Cookie Overview

| Cookie | Set By | Purpose | Lifetime |
|--------|--------|---------|----------|
| `JSESSIONID` | Client (generated) or server | CSRF token | Short-lived (max-age: 100) but regenerated as needed |
| `li_at` | Server (`/uas/authenticate`) | Session authentication | Long-lived (weeks/months) |
| Other LinkedIn cookies | Server | Various tracking/preferences | Varies |

### 5.2 li_at Cookie

- **Set by**: Server response to `POST /uas/authenticate` (via `Set-Cookie` header)
- **Domain**: `.linkedin.com`
- **Used for**: All authenticated API requests
- **Checked by**: `UnauthorizedStatusCodeHandler` on HTTP 401
- **Validated by**: `LiSSOService.isValidSignedInUser()` -- checks both username contains `@` AND `li_at` cookie exists

### 5.3 Cookie Storage

The auth library uses `AuthHttpStackWrapper` which wraps the app's `HttpStack`. Cookies are stored in the networking layer's `LinkedInHttpCookieManager` (custom implementation, NOT Android's CookieManager).

Key operations:
- `addCookie(String)` -- parse and store a cookie string
- `getCookie(String name, String host)` -- get cookie value by name and host
- `getCookieNameValuePairs(Set<String> domains)` -- get all cookie name-value pairs for given domains
- `clearAuthCookies()` -- clear all auth cookies (on logout)
- `getCookies()` -- get all cookies as strings (used by SSO token sharing)
- `needsWebViewCookieSync()` -- whether WebView cookies need syncing
- `addCookiesToCookieManager(CookieManager)` -- sync cookies to Android WebView CookieManager

### 5.4 needsAuth Check

`LiAuthImpl.needsAuth()` simply checks if `getUsername()` returns a non-empty string. Username is stored in `SharedPreferences` key `"auth_username"`. This does NOT check cookie validity -- a user can have a username stored but an expired `li_at` cookie.

---

## 6. Session Validation and 401 Handling

Source: `com.linkedin.android.app.UnauthorizedStatusCodeHandler`

When any API request returns HTTP 401:

1. Check guards:
   - `HANDLE_UNAUTHORIZED_STATUS_CODE` must be true (set to false after first 401 to prevent cascading logouts)
   - User must be currently authenticated (`auth.isAuthenticated()`)
   - Request URL must start with the base URL (don't handle 401 from third-party URLs)

2. Enumerate all cookies for the request URL, looking specifically for `li_at`

3. **If `li_at` exists AND feature flag `UNAUTHORIZED_LOGOUT_ONLY_WHEN_NO_LIAT` is enabled**:
   - Report non-fatal error only (don't log out)
   - Log: "Unexpected 401" with TreeId and cookie list

4. **If `li_at` is missing**:
   - Log: "Logging user out due to missing li_at cookie"
   - Proceed to logout

5. **If `li_at` exists but flag is NOT enabled**:
   - Log: "Logging user out due to 401 response"
   - Proceed to logout

6. Logout behavior depends on app state:
   - **Background**: `auth.signOut()` with reason `UNAUTHORIZED`
   - **Foreground**: Navigate to `LoginActivity` with logout reason

**Key insight**: There is NO automatic token refresh. A 401 means the session is dead.

---

## 7. Logout Flow

Source: `LiAuthImpl.logoutInternal()`

### 7.1 Sequence

```
POST {baseHost}/uas/directLogout
Content-Type: application/x-www-form-urlencoded
Body: {all LinkedIn domain cookies}&logout_reason={USER_INITIATED|UNAUTHORIZED}
```

1. If SSO is active: notify all bound SSO services to sign out
2. Collect all cookies for LinkedIn domains as form params
3. Add `logout_reason` param (`USER_INITIATED` or `UNAUTHORIZED`)
4. `POST /uas/directLogout`
5. If response != 200: clear auth cookies locally anyway
6. Clear user info from SharedPreferences
7. Clear enterprise user info
8. Clear cached profile picture
9. Broadcast empty member ID (`"ACTIVE_USER_INTENT_ACTION"` with empty string)

---

## 8. Third-Party OAuth / Mobile SDK Flow (PKCE)

This is for OTHER APPS requesting LinkedIn authorization tokens -- NOT for the LinkedIn app itself. However, the endpoint structure is relevant.

### 8.1 Authorization URL

Source: `LiThirdPartyWebViewAuthorizeActivity.getOAuthUrl()`

```
GET {baseHost}/oauth/mobilesdk/authorization
  ?response_type=code
  &client_id={callerPackageName}
  &packageHash={SHA256 of caller's signing certificate}
  &scope={requested_scopes}
  &redirect_uri={caller_redirect_uri}
  &code_challenge={PKCE_challenge}
  &code_challenge_method=S256
  &state={random_state}
  &_l={locale}
```

Note: `client_id` is the calling app's package name (e.g., `com.android.contacts`), NOT a traditional OAuth client ID.

### 8.2 Authorization Grant

Source: `LiThirdPartyAuthorizer.onAuthenticated()`

```
POST {baseHost}/uas/mobilesdk/authorize
Content-Type: application/x-www-form-urlencoded
Headers:
  Locale: {locale}
  X-isAJAXForm: 1

Body:
  scope={scopes}
  &locale={locale}
  &duid={device_unique_id}
  &packageName={caller_package}
  &packageHash={caller_cert_hash}
  &csrfToken={JSESSIONID_value}
  &userAuthorized={true|false}
  [&sd={server_data}]
```

Response: JSON containing authorization code, app name, permissions info.

### 8.3 Token Creation (for Samsung SSO)

Source: `OAuthNetworkHelper.getToken()`

```
POST /uas/oauth2/createToken
Content-Type: application/x-www-form-urlencoded
Headers:
  X-IsAJAXForm: 1

Body:
  client_id={api_key}
  &client_secret={secret}
  &scope={scope}
```

Response:
```json
{
  "status": "ok",
  "content": {
    "expires_in": 86400,
    "access_token": "AQV..."
  }
}
```

### 8.4 Token Revocation

```
POST /uas/oauth2/revokeToken
Content-Type: application/x-www-form-urlencoded
Headers:
  X-IsAJAXForm: 1

Body:
  oauth2Token={token_to_revoke}
```

### 8.5 Approved Samsung API Keys

The `OAuthService` has a static set `APPROVED_SAMSUNG_APIKEYS` and `APPROVED_SAMSUNG_SIGNATURES` -- Samsung is a privileged partner for LinkedIn SSO on Samsung devices.

---

## 9. SSO Service (Inter-App Token Sharing)

Source: `com.linkedin.android.liauthlib.sso.LiSSOService`

### 9.1 Architecture

The LinkedIn app exposes an Android bound `Service` that other LinkedIn-signed apps can bind to:

**Intent filter**:
- Actions: `com.linkedin.android.auth.GET_ACCOUNTS`, `com.linkedin.android.auth.GET_TOKENS`
- Category: `com.linkedin.android.auth.SSO`

### 9.2 Security: Signature Verification

`LiSSOService.verifyCallerSignature()`:
1. Get the caller's UID from `Binder.getCallingUid()`
2. Resolve to package name via `PackageManager.getPackagesForUid()`
3. Get the package's signing certificate
4. Check if the certificate's hex string is in `approvedSignatures` (a static `TreeSet`)
5. Only approved signatures can access SSO data

### 9.3 IAuthService Interface

The bound service implements `IAuthService` (AIDL interface):

| Method | Returns |
|--------|---------|
| `getSignedInUser(host)` | `Map<username, packageName>` if user logged in with email |
| `getSignedInUserWithProfileData(host)` | User profile data map |
| `getEmailAndPhoneSignedInUserWithProfileData(host)` | Same but includes phone-login users |
| `getSSOUsers(host, includePhone, includeEnterprise)` | Filtered SSO users |
| `getTokensForUser(username)` | `List<String>` of all cookies (if username matches signed-in user) |
| `getProfilePicForUser(username)` | `Bitmap` profile picture |
| `signout()` | Sign out the current user (broadcasts `EXTERNAL_SSO_LOGOUT_ACTION`) |

### 9.4 Session Validity for SSO

`isValidSignedInUser(username, host, httpStack)`:
- Username must be non-empty
- Username must contain `@` (email format required for SSO)
- `li_at` cookie must exist for the given host

### 9.5 Token Sharing

When another app requests tokens via `getTokensForUser()`:
1. Verify caller signature
2. Check if requested username matches the signed-in user
3. Return ALL cookies from the HTTP stack (not just li_at)

This means SSO consumers get the full cookie set including JSESSIONID.

---

## 10. Alternative Login Methods

### 10.1 Google Sign-In

Parameter: `googleIdToken` in POST body to `/uas/authenticate`

Google client ID (from `RegistrationHelper`):
```
audience:server:client_id:789113911969.apps.googleusercontent.com
```

The app gets a Google ID token via Google Sign-In SDK, then passes it directly to `/uas/authenticate`. LinkedIn server validates the token server-side.

### 10.2 Apple Sign-In

Parameters: `appleIdToken` + `appleAuthCode` in POST body to `/uas/authenticate`

Apple sign-in uses a WebView flow (`webview_authentication_type: "apple_sign_in"`) with a bridge interface (`AppleSignInWebViewInterface`) to communicate between the WebView and the native app.

Result: `LiAppleAuthResponse` with status `SUCCESS`, `ERROR`, or other `ResponseResult` values.

### 10.3 Mid-Token Login (Fastrack)

Parameter: `session_midToken` in POST body to `/uas/authenticate`

Used for "fastrack" login -- user already has a mid-token (from a link, QR code, etc.).

Post-login profile fetch: `POST {baseHost}/checkpoint/login/fastrackProfileV2`
- Content-Type: application/json
- Body: `{"midToken": "{mid_token}"}`
- Headers: `Csrf-Token`, `X-LI-Track`

### 10.4 Flash ID Login (China/Zephyr)

Parameters: `flashIdToken` + `flashAuthInfoId` in POST body to `/uas/authenticate`

Used for carrier-based instant login in Chinese market (Shanyan SDK integration).

### 10.5 One-Click Login

Source: `LiAuthImpl.authenticateWithLoginToken()` and `TokenManager`

```
POST {baseHost}/checkpoint/login-with-token
Content-Type: application/json
Body: {"applicationToken": "{uuid}", "loginToken": "{token_from_url}"}
```

The `applicationToken` is a UUID generated by `TokenManager`:
1. Generate `UUID.randomUUID()`
2. Store as `applicationToken`
3. Hash with SHA-256 and Base64-encode as `hashedApplicationToken`
4. The hashed version is used for the Custom Tabs URL; the raw UUID is sent to the server

### 10.6 Remember Me

Uses WebView flow with `webview_authentication_type: "remember_me"`. The WebView bridge returns one of:
- `SIGN_IN` -> navigate to login
- `JOIN_NOW` -> navigate to registration
- Direct auth success -> call `authenticateAfterChallenge()`

---

## 11. Registration Flow

Source: `RegistrationHelper`

### 11.1 Account Creation

Two URL patterns:
- New: `{baseHost}/signup/api/createAccount?{params}`
- Legacy: `{baseHost}/start/reg/api/createAccount?{params}`

### 11.2 SMS Pin Verification

For phone-number registration:
1. `sendSMSPin()` -- send verification code to phone
2. `verifySMSPin()` -- verify the entered code

---

## 12. Request Headers During Auth

### 12.1 Auth Library User-Agent

Source: `NetworkUtils.getXliUserAgent()`

```
LIAuthLibrary:0.0.3 com.linkedin.android:{versionName} {manufacturer}_{model}:android_{release}
```

Example: `LIAuthLibrary:0.0.3 com.linkedin.android:4.1.940 Google_Pixel 6:android_13`

### 12.2 X-LI-Track Header

Source: `NetworkUtils.getXLiTrackHeader()`

JSON object sent during auth requests:

```json
{
  "appId": "com.linkedin.android",
  "carrier": "Google",
  "clientVersion": "4.1.940",
  "clientMinorVersion": "12345",
  "deviceType": "android",
  "dpi": "xxhdpi",
  "language": "en_US",
  "model": "Google_Pixel 6",
  "osName": "ANDROID OS/13",
  "osVersion": "13",
  "timezone": "-8"
}
```

### 12.3 Default Headers

Source: `NetworkUtils.getDefaultHeaders()`

```
X-LI-User-Agent: LIAuthLibrary:0.0.3 com.linkedin.android:...
User-Agent: ANDROID OS
Accept-Language: en-US
```

---

## 13. Host Configuration

### 13.1 Host Resolution

Source: `LiAuthImpl.resolveHost()`

| LiAuthHost | URL |
|------------|-----|
| `PROD` | `https://www.linkedin.com` |
| `EI` / `EI2` | `https://www.linkedin-ei.com` |
| `CUSTOM` | Whatever string was provided |

Default: `https://www.linkedin.com` (stored in SharedPreferences key `"auth_selected_host"`)

The `Auth` facade overrides the host: it reads `authUrl` from `FlagshipSharedPreferences` and sets it as CUSTOM host before every sign-in.

### 13.2 Cookie Domain Specifications

Two sets of domains for cookie collection:

**Standard (`cookieDomainSpec`)**: `.linkedin-ei.com`, `linkedin-ei.com`, `.www.linkedin-ei.com`, `www.linkedin-ei.com`, `.linkedin.com`, `linkedin.com`, `.www.linkedin.com`, `www.linkedin.com`

**Full hostname (`liFullHostnameCookieDomainSpec`)**: Same as above plus `https://www.linkedin.com`, `https://www.linkedin-ei.com`

Which set is used depends on the `WHITELIST_LI_DOMAIN_LIX` feature flag.

---

## 14. Biometric / App Lock

Source: `BiometricAuthManager`, `DeviceCredentialVerificationActivity`

- Initialized on API 23+ (Android 6.0+)
- Uses Android BiometricPrompt / device credentials
- Not part of the login flow -- this is for app lock (re-verify identity when resuming the app)
- Has its own feature flag: `APP_LOCK`

---

## 15. Enterprise Auth

Source: `com.linkedin.android.liauthlib.enterprise.EnterpriseAuth`

Separate enterprise authentication path exists. Enterprise users are tracked via:
- `LiSSOInfo.isEnterpriseUser()` -- SSO filtering respects this
- `clearEnterpriseUserInfo()` -- cleared on logout
- Enterprise users may be excluded from SSO sharing depending on caller preferences

---

## 16. Key Insights for Rust Client Implementation

### 16.1 Minimal Auth Sequence

For a Rust client that wants to authenticate:

1. **Generate JSESSIONID**: `"ajax:" + format!("{:019}", rand::random::<u64>() % 10u64.pow(19))`
2. **Set JSESSIONID as cookie** for `www.linkedin.com`
3. **POST** to `https://www.linkedin.com/uas/authenticate`:
   ```
   Content-Type: application/x-www-form-urlencoded
   Cookie: JSESSIONID=ajax:0123456789012345678

   session_key={email}&session_password={password}&JSESSIONID=ajax:0123456789012345678
   ```
   Note: The JSESSIONID is included BOTH as a cookie AND as a form parameter (because the code appends all cookie name-value pairs to the form body).

4. **Check response**:
   - 200 with `{"login_result": "PASS"}` -> success, extract `li_at` from `Set-Cookie`
   - 401 with `CHALLENGE` -> need CAPTCHA handling (probably manual)
   - 401 with `BAD_PASSWORD` or `BAD_EMAIL` -> invalid credentials

5. **For subsequent API requests**, include:
   ```
   Cookie: li_at={value}; JSESSIONID=ajax:{value}
   Csrf-Token: ajax:{value}
   ```

### 16.2 Session Keepalive

There is NO explicit keepalive or token refresh. The `li_at` cookie is long-lived. If it expires:
- API calls return 401
- The app logs out and requires re-authentication
- There is no "refresh token" concept

### 16.3 Critical Cookies to Preserve

At minimum, the cookie jar must maintain:
- `li_at` -- THE session cookie
- `JSESSIONID` -- CSRF token (can be regenerated, but must be consistent within a session)

### 16.4 Rate Limiting / Anti-Automation

- The `client_enabled_features=ANDROID_NATIVE_CAPTCHA` parameter tells LinkedIn the client supports native CAPTCHA
- Google reCAPTCHA site key: `6LdcXFUUAAAAANu32LPaK4zJiXYrjHj4efRGaMFu`
- Automated login will likely trigger CHALLENGE responses
- The `X-LI-Track` header with device info helps avoid detection

### 16.5 What We Don't Need

- OAuth2 bearer tokens (first-party app doesn't use them)
- PKCE flow (that's for third-party apps)
- Samsung SSO (that's for Samsung partner integration)
- Biometric auth (that's for app lock, not login)

---

## 17. Endpoint Reference

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/uas/authenticate` | GET | Obtain JSESSIONID cookie (CSRF bootstrap) |
| `/uas/authenticate` | POST | Login with credentials |
| `/uas/issueLoginCookie` | POST | Convert WebView cookies to app cookies |
| `/uas/directLogout` | POST | Logout |
| `/mob/sso/you` | GET | Fetch profile data + validate session |
| `/checkpoint/login/fastrackProfileV2` | POST | Fastrack profile fetch (mid-token) |
| `/checkpoint/login-with-token` | POST | One-click login |
| `/checkpoint/challenges/nativeCaptchaChallenge/{id}` | POST | Submit native CAPTCHA response |
| `/oauth/mobilesdk/authorization` | GET | Third-party OAuth authorization (PKCE) |
| `/uas/mobilesdk/authorize` | POST | Third-party OAuth grant |
| `/uas/oauth2/createToken` | POST | Create OAuth2 token (Samsung partner) |
| `/uas/oauth2/revokeToken` | POST | Revoke OAuth2 token |
| `/signup/api/createAccount` | POST | New account registration |
| `/start/reg/api/createAccount` | POST | Legacy account registration |

---

## 18. Open Questions

1. **li_at cookie expiry**: What is the actual `Max-Age` or `Expires` value set by the server? This needs live validation.
2. **Additional cookies**: Are there other session cookies beyond `li_at` that are required for API calls? Live traffic capture would clarify.
3. **CAPTCHA avoidance**: How frequently does LinkedIn trigger CHALLENGE on mobile logins? Does proper `X-LI-Track` header reduce this?
4. **Cookie rotation**: Does the server ever rotate the `li_at` value during a session, or is it fixed until expiry?
5. **Rate limits**: What are the rate limits on `/uas/authenticate`? How many failed attempts trigger a lockout?
