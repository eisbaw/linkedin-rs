---
id: TASK-0031
title: Validate search endpoint — GET /voyager/api/search/hits
status: To Do
assignee: []
created_date: '2026-03-24 07:49'
labels: []
dependencies:
  - TASK-0026
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Call the search endpoint with a simple keyword query. Verify facet encoding, response format, and pagination. Compare against search models from re/search_protocol.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Search endpoint returns results for keyword query
- [ ] #2 Response structure compared against documented models
- [ ] #3 Facet parameters work
- [ ] #4 Response saved as fixture in secrets/
<!-- AC:END -->
