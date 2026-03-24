---
id: TASK-0024
title: Implement HTTP client with LinkedIn-specific headers and cookie jar
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 07:49'
updated_date: '2026-03-24 08:05'
labels: []
dependencies:
  - TASK-0023
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build the core HTTP client in linkedin-api that sets all required headers: X-RestLi-Protocol-Version 2.0.0, Csrf-Token (from JSESSIONID cookie), X-LI-Track (device telemetry JSON), X-UDID, User-Agent. Cookie jar with reqwest cookie_store(true). Accept: application/json. Per re/architecture_overview.md, re/device_fingerprinting.md, re/restli_protocol.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 HTTP client sets all required headers on every request
- [x] #2 JSESSIONID generated in correct format (ajax:{19-digit})
- [x] #3 X-LI-Track JSON matches documented format
- [x] #4 Cookie jar persists across requests
- [x] #5 Accept header requests JSON (not protobuf)
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented LinkedInClient in client.rs with:
- reqwest::Client with cookie_store(true) via cookie_provider(Arc<Jar>)
- JSESSIONID generated as ajax:{19-digit zero-padded abs(random i64)}, seeded into cookie jar on .linkedin.com
- X-LI-Track JSON (18 fields) built with realistic Pixel 8 values, dynamic timezone
- X-UDID header == deviceId in X-LI-Track (consistency requirement)
- Csrf-Token header set per-request to JSESSIONID value
- X-RestLi-Protocol-Version: 2.0.0 on every request
- Accept: application/json (not protobuf)
- get(path) and post(path, body) methods prepend /voyager/api/
- with_identity() constructor for session persistence
- 6 unit tests + 3 pre-existing smoke tests all pass
- Dependencies added: uuid v1 (v4 feature), iana-time-zone v0.1
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
HTTP client fully implemented with all LinkedIn-specific headers, cookie jar, JSESSIONID generation, and X-LI-Track device telemetry. All e2e gates pass (build, test, clippy, fmt-check).
<!-- SECTION:FINAL_SUMMARY:END -->
