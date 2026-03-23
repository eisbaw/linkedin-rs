# PRD: LinkedIn Android App Reverse Engineering

## Goal

Reverse-engineer the LinkedIn Android app (`com.linkedin.android`) to build a production-quality Rust CLI and API library for programmatic access to LinkedIn's core features.

## Scope

### In Scope
- Authentication flow (OAuth2/OIDC, session management)
- Profile viewing (own profile, connections, public profiles)
- Messaging (read, send, conversations)
- Feed/posts (read, create, react)
- Connections (list, invitations, search)
- Notifications (read, mark as read)
- Job listings (search, view, saved jobs)

### Out of Scope (for now)
- LinkedIn Learning
- Sales Navigator
- Recruiter features
- LinkedIn Ads
- Write-heavy operations that could trigger account restrictions

## Deliverables

1. **`linkedin-api`** Rust crate — typed HTTP client with auth, session management, and models
2. **`linkedin-cli`** Rust binary — clap-based CLI with subcommands per domain
3. **`linkedin-fuse`** (optional) — FUSE filesystem exposing LinkedIn data as files

## Milestones

### M1: Extraction & Decompilation
- Obtain APK via apkeep
- Extract and decompile (jadx, apktool)
- Document app architecture

### M2: Static Analysis
- Map all API endpoints
- Document auth flow (OAuth2, cookies, CSRF)
- Extract data models/DTOs
- Identify API versioning and transport (REST vs LinkedIn's Restli)

### M3: Live API Validation
- Authenticate against live API
- Call discovered endpoints, capture real responses
- Fix model mismatches between decompiled code and actual API

### M4: Rust Library
- HTTP client with cookie jar, auth decoration
- Auth module (OAuth2 + CSRF + session)
- Typed models per domain
- Service layer per domain

### M5: CLI
- Clap subcommands per domain
- Human-readable + JSON output
- Showcase recipe

### M6: Testing & Hardening
- E2E tests against live API
- Fixture tests from captured responses
- PII scan and cleanup

### M7: Advanced Features (optional)
- FUSE filesystem
- Lazy media download
- Export to standard formats

## Known Challenges

- LinkedIn uses a custom REST framework called **Rest.li** (not standard REST)
  - Requests use a specific header protocol (`X-RestLi-Protocol-Version`, `X-RestLi-Method`)
  - Response format wraps data in `elements` arrays with pagination metadata
- LinkedIn actively detects and blocks automated access
  - Rate limiting, CAPTCHA challenges, account restrictions
  - User-Agent and TLS fingerprint checking
- Auth flow likely involves:
  - OAuth2 with PKCE for mobile
  - CSRF token (`jsessionid` cookie echoed as header)
  - Possible li_at cookie-based session
- API versioning: LinkedIn has migrated between v1 and v2 APIs; the app may use both
- Heavy ProGuard/R8 obfuscation expected

## Security Rules

- No tokens, credentials, or PII in tracked files
- All secrets go in `secrets/` (gitignored)
- PII scan before any push to remote
- Rate-limit API calls to avoid account restrictions
- This is for personal/educational use only
