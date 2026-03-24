---
id: TASK-0024
title: Implement HTTP client with LinkedIn-specific headers and cookie jar
status: To Do
assignee: []
created_date: '2026-03-24 07:49'
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
- [ ] #1 HTTP client sets all required headers on every request
- [ ] #2 JSESSIONID generated in correct format (ajax:{19-digit})
- [ ] #3 X-LI-Track JSON matches documented format
- [ ] #4 Cookie jar persists across requests
- [ ] #5 Accept header requests JSON (not protobuf)
<!-- AC:END -->
