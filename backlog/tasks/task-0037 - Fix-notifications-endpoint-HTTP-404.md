---
id: TASK-0037
title: Fix notifications endpoint (HTTP 404)
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 09:22'
updated_date: '2026-03-24 10:09'
labels: []
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GET /voyager/api/identity/notificationCards returns 404 (HTML error page, not JSON). The endpoint path is wrong. Check decompiled/jadx_intl/ for the correct notifications endpoint — may need a different path or query parameters. The international build likely uses a Dash endpoint.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Notifications endpoint returns 200 with notification data
- [x] #2 CLI notifications list shows notifications correctly
- [x] #3 Response saved as fixture in secrets/
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Replace get_notifications() in client.rs: switch from REST identity/notificationCards to GraphQL using identityDashNotificationCardsByFilterVanityName finder (queryId voyagerIdentityDashNotificationCards.1a1ca07d1f7a6e1033fd88d5fd2da611)
2. Unwrap GraphQL envelope (data.identityDashNotificationCardsByFilterVanityName) to return the collection
3. Update CLI cmd_notifications_list to handle updated response shape
4. Test against live API
5. Save response fixture to secrets/notifications_response.json
6. Run just e2e
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Root cause: REST endpoint identity/notificationCards returns 404 (removed server-side in intl build)
- Fix: Switched to GraphQL identityDashNotificationCardsByFilterVanityName finder (queryId voyagerIdentityDashNotificationCards.1a1ca07d1f7a6e1033fd88d5fd2da611)
- Discovered via NotificationsGraphQLClient.java in decompiled intl APK
- Tested live: returns HTTP 200 with real notification data
- CLI human-readable and --json output both work correctly
- Response saved to secrets/notifications_response.json (1273 lines)
- e2e gate passes (37 tests, clippy clean, fmt clean)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed notifications endpoint HTTP 404 by migrating from deprecated REST path to GraphQL Dash endpoint.

Changes:
- Replaced get_notifications() in linkedin-api/src/client.rs: switched from GET /voyager/api/identity/notificationCards (404) to GraphQL identityDashNotificationCardsByFilterVanityName finder query
- Updated doc comment in linkedin-cli/src/main.rs to reflect new endpoint
- No model changes needed -- the GraphQL envelope is unwrapped to the same collection shape (elements/paging/metadata)

Discovery:
- Found correct endpoint in decompiled NotificationsGraphQLClient.java (jadx_intl)
- queryId: voyagerIdentityDashNotificationCards.1a1ca07d1f7a6e1033fd88d5fd2da611
- Same pattern as profile and search migrations (REST -> Dash/GraphQL)

Tested:
- Live API returns HTTP 200 with notification card data
- CLI --json and human-readable output both render correctly
- Response fixture saved to secrets/notifications_response.json
- e2e gate passes (37 tests, clippy, fmt)
<!-- SECTION:FINAL_SUMMARY:END -->
