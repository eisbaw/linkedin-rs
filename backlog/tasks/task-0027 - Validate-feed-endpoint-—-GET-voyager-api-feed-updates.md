---
id: TASK-0027
title: Validate feed endpoint — GET /voyager/api/feed/updates
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
Call the feed endpoint with decoration/recipe parameters. Verify pagination works (start, count). Compare response structure against Pegasus models documented in re/pegasus_models.md. Fix any model mismatches.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Feed endpoint returns paginated results
- [ ] #2 Response structure matches or diverges documented — differences noted
- [ ] #3 Pagination parameters work (start, count)
- [ ] #4 Response saved as fixture in secrets/
<!-- AC:END -->
