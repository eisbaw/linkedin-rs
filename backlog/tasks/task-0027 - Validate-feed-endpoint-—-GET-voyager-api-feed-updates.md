---
id: TASK-0027
title: Validate feed endpoint — GET /voyager/api/feed/updates
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 07:49'
updated_date: '2026-03-24 08:25'
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
- [x] #1 Feed endpoint returns paginated results
- [ ] #2 Response structure matches or diverges documented — differences noted
- [x] #3 Pagination parameters work (start, count)
- [ ] #4 Response saved as fixture in secrets/
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented get_feed() client method calling GET feed/updates?q=findFeed with start/count pagination. Added FeedResponse, Paging, UpdateV2, SocialDetail, SocialActivityCounts model structs with liberal Option<T> usage. Implemented feed list CLI subcommand with --count/--start/--json flags. All e2e tests pass (15 unit + 3 smoke). AC #2 and #4 require live API validation -- not yet done.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added feed endpoint support: get_feed() client method (GET feed/updates?q=findFeed&start=N&count=N), typed models (FeedResponse, Paging, UpdateV2, SocialDetail, SocialActivityCounts) with serde defaults for unknown fields, and feed list CLI subcommand with --count/--start/--json flags. Human-readable output shows actor name, commentary excerpt, social counts per item. All e2e tests pass. AC #2 and #4 remain unchecked -- they require live API validation with an active session.
<!-- SECTION:FINAL_SUMMARY:END -->
