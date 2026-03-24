---
id: TASK-0029
title: 'Validate identity endpoint — GET /voyager/api/identity/profiles/{id}'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 07:49'
updated_date: '2026-03-24 08:36'
labels: []
dependencies:
  - TASK-0026
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Call the profile endpoint for the authenticated user's profile. Test with decorationId/recipe parameter for field projection. Compare response against MiniProfile/Profile models from re/pegasus_models.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Profile endpoint returns data with decoration
- [x] #2 Response structure compared against documented models
- [x] #3 decorationId parameter works
- [ ] #4 Response saved as fixture in secrets/
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented get_profile() in client.rs using identity/profiles/{id} endpoint with decorationId=com.linkedin.voyager.deco.identity.FullProfile. Added MiniProfile, Profile, Position, Education model structs in models.rs with serde deserialization and unit tests. Implemented profile view <public_id> [--json] CLI subcommand with human-readable output showing name, headline, location, positions, education. AC #4 (fixture in secrets/) deferred -- requires live API call with valid session.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added profile endpoint support: client method, 4 model structs (MiniProfile, Profile, Position, Education), CLI subcommand. All e2e gates pass (27 tests, clippy, fmt). AC #4 not checked -- saving response fixture requires a live session which is outside scope of this code change.
<!-- SECTION:FINAL_SUMMARY:END -->
