---
id: TASK-0029
title: 'Validate identity endpoint — GET /voyager/api/identity/profiles/{id}'
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
Call the profile endpoint for the authenticated user's profile. Test with decorationId/recipe parameter for field projection. Compare response against MiniProfile/Profile models from re/pegasus_models.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Profile endpoint returns data with decoration
- [ ] #2 Response structure compared against documented models
- [ ] #3 decorationId parameter works
- [ ] #4 Response saved as fixture in secrets/
<!-- AC:END -->
