---
id: TASK-0007
title: Map complete API endpoint catalog from decompiled code
status: Done
assignee:
  - '@claude'
created_date: '2026-03-23 23:00'
updated_date: '2026-03-24 05:41'
labels:
  - phase3
  - static-analysis
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Extract all API endpoints from the decompiled code beyond what Routes.java provides. Search for URL construction patterns in DataProvider classes, direct URL strings in feature packages (feed, messaging, jobs, identity, search, etc.), and any dynamically constructed endpoints. Produce a comprehensive endpoint catalog in re/api_endpoints.md with HTTP methods, parameters, request/response models, and decoration IDs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Complete endpoint catalog documented in re/api_endpoints.md
- [x] #2 Endpoints organized by domain (identity, feed, messaging, jobs, search, etc.)
- [x] #3 Each endpoint includes HTTP method, path, query parameters, and linked Pegasus model
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Extracted all 375 route enum constants from Routes.java (lines 25-399).
Cross-referenced with SearchRoutes.java, ProfileRoutes.java, MessagingRoutes.java, JobsRouteUtils.java, FeedRouteUtils.java, FeedActionRouteUtils.java, PublishingRouteUtils.java, MyNetworkRoutesUtil.java, EntityRouteUtils.java.
Inferred HTTP methods from DataRequest.get()/post()/put()/delete() calls in DataProvider/Repository classes.
Extracted 24 decoration/recipe IDs from appendRecipeParameter() calls.
Organized 20 domain sections with full query parameters, HTTP methods, and Pegasus model references.
Output written to re/api_endpoint_catalog.md (note: task AC says re/api_endpoints.md but the catalog is at re/api_endpoint_catalog.md per the user request).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Complete API endpoint catalog extracted from decompiled LinkedIn Android app.

Changes:
- Created re/api_endpoint_catalog.md with 375+ route definitions organized into 20 domain sections
- Each endpoint documents: HTTP method, URL path, query parameters, decoration/recipe IDs, and associated Pegasus models
- Domains covered: auth, identity/profile, feed, content creation, messaging, jobs, connections/network, search, typeahead, notifications, organizations, premium, growth/onboarding, groups, real-time, media upload, tracking, geographic, and Zephyr-specific
- Appendices cover: non-API-prefix endpoints, key recipe IDs (24 extracted), pagination conventions, and Rest.li action patterns

Sources analyzed:
- Routes.java (375 enum constants)
- SearchRoutes.java, ProfileRoutes.java, MessagingRoutes.java, JobsRouteUtils.java, FeedRouteUtils.java, FeedActionRouteUtils.java, PublishingRouteUtils.java, MyNetworkRoutesUtil.java, EntityRouteUtils.java
- 50+ DataProvider/Repository classes for HTTP method inference
<!-- SECTION:FINAL_SUMMARY:END -->
