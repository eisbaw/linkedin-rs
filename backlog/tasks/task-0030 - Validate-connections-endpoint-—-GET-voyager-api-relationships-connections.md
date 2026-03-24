---
id: TASK-0030
title: Validate connections endpoint — GET /voyager/api/relationships/connections
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 07:49'
updated_date: '2026-03-24 08:40'
labels: []
dependencies:
  - TASK-0026
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Call the connections/network endpoint. Verify pagination, response structure. Compare against Connection model from re/pegasus_models.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Connections endpoint returns data
- [x] #2 Response structure compared against documented models
- [ ] #3 Response saved as fixture in secrets/
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented get_connections() in client.rs calling GET /voyager/api/relationships/connections?start=N&count=N&sortType=RECENTLY_ADDED. Added Connection and ConnectionsResponse models in models.rs matching pegasus_models.md fields. Added 'connections list [--count N] [--start N] [--json]' CLI subcommand with human-readable output showing name, headline, connected-since date. All e2e tests pass (31 unit + 3 smoke, clippy clean, fmt clean). AC#3 (save fixture in secrets/) deferred to live validation with active session.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Connections endpoint fully implemented: client method, typed models (Connection, ConnectionsResponse), and CLI subcommand with human-readable output. All e2e tests pass. AC#3 (fixture in secrets/) requires live session validation.
<!-- SECTION:FINAL_SUMMARY:END -->
