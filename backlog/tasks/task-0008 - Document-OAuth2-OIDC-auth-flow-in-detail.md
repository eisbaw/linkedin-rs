---
id: TASK-0008
title: Document OAuth2/OIDC auth flow in detail
status: Done
assignee:
  - '@claude'
created_date: '2026-03-23 23:00'
updated_date: '2026-03-24 05:39'
labels:
  - phase3
  - static-analysis
  - auth
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Deep-dive into the authentication flow from LiAuthImpl, LiAuth, AuthHttpStackWrapper, and related classes. Document the complete cookie-based auth flow, CSRF token generation, session lifecycle, token refresh, third-party OAuth PKCE flow, Google/Apple/Flash sign-in variants, and challenge/CAPTCHA handling. This is critical for implementing auth in the Rust client.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Complete auth flow documented in re/auth_flow.md
- [x] #2 Cookie lifecycle documented (li_at, JSESSIONID, others)
- [x] #3 CSRF token generation and validation documented
- [x] #4 Third-party OAuth PKCE flow documented with all parameters
- [x] #5 Challenge/CAPTCHA handling flow documented
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Traced full auth flow through decompiled jadx sources:
- LiAuthImpl.authenticate() -> NetworkUtils.performRequestWithCSRFToken() -> POST /uas/authenticate
- CsrfCookieHelper.generateJsessionId() -> "ajax:{19-digit random}"
- UnauthorizedStatusCodeHandler checks li_at cookie on 401
- No token refresh mechanism exists - li_at is long-lived, 401 = full re-auth
- Third-party OAuth uses PKCE with /oauth/mobilesdk/authorization
- Challenge handling: WebView for general challenges, SafetyNet reCAPTCHA for native CAPTCHA
- Samsung has privileged SSO via OAuthService with /uas/oauth2/createToken
- LiSSOService exposes IAuthService AIDL for inter-app cookie sharing
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Documented the complete LinkedIn Android auth flow in re/auth_flow.md (18 sections, ~500 lines).

Key findings:
- Auth is entirely cookie-based (li_at + JSESSIONID), no bearer tokens for first-party
- JSESSIONID format: "ajax:{19-digit SecureRandom}", generated client-side by CsrfCookieHelper
- Login: POST /uas/authenticate with form-encoded credentials + all existing cookies as form params
- No token refresh exists -- 401 means re-authenticate from scratch
- Challenge/CAPTCHA: WebView fallback or native SafetyNet reCAPTCHA (site key documented)
- Third-party OAuth uses PKCE via /oauth/mobilesdk/authorization (package name as client_id)
- SSO via bound Android Service with AIDL interface, signature-verified callers
- Samsung has privileged token creation via /uas/oauth2/createToken
- Documented all login_result values, error codes, cookie domains, headers, and endpoint reference

This provides everything needed to implement auth in the Rust client.
<!-- SECTION:FINAL_SUMMARY:END -->
