---
id: TASK-0026
title: Validate auth against live API — call /voyager/api/me endpoint
status: To Do
assignee: []
created_date: '2026-03-24 07:49'
labels: []
dependencies:
  - TASK-0025
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
First live API test. Use the authenticated client to call GET /voyager/api/me (or equivalent profile endpoint). Verify: JSON response received (not protobuf rejection), response deserializes, session cookie works. Save response to secrets/ as test fixture. Per re/api_endpoint_catalog.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GET /voyager/api/me returns valid JSON
- [ ] #2 Response saved as test fixture in secrets/
- [ ] #3 Any header/auth issues documented and fixed
- [ ] #4 Confirm application/json Accept header is honored
<!-- AC:END -->
