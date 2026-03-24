---
id: TASK-0019
title: Document LIX feature flag system and endpoint
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 06:29'
updated_date: '2026-03-24 06:58'
labels: []
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Read LixManager, LixNetworkManager, LixDefinition, and LixOverrideCookieHelper to document the feature flag request/response format, caching, and override mechanism. Package: com.linkedin.android.lixclient
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 LIX endpoint URL and HTTP method documented
- [x] #2 Request/response format documented
- [x] #3 Override mechanism documented
- [x] #4 Findings written to re/lix_feature_flags.md
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Analyzed decompiled source files: LixNetworkManager, LixManagerImpl, LixV2BatchGetFactory, LixV3BatchGetFactory, LixCacheManager, LixMemoryCache, LixDiskCache, LixOverrideManager, LixOverrideCookieHelper, LixTreatment, LixBatchGetContext, LixBatchGetContextV3, AuthenticatedLixManagerImpl, GuestLixManagerImpl, Lix enum, GuestLix enum.

Key finding: endpoint is POST /lix/lixFrontendTreatmentsV2?action=batchGet (Rest.li action), NOT under /voyager/api/ prefix. Wire format is custom protobuf with symbol tables by default, JSON as fallback.

Override mechanism uses lror cookie (10-day expiry) with key=value&key=value format, checked before server treatments in getTreatment() priority chain.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Documented the complete LIX feature flag system including: V2/V3 endpoints (POST /lix/lixFrontendTreatments{V2,V3}?action=batchGet), request body format (LixBatchGetContext with keys, targetIdentityContext, evaluationContext), response format (List<LixTreatment> with testKey, treatment, trackingInfo), two-tier caching (memory + LMDB disk), lror cookie override mechanism, periodic 15-minute sync schedule, and consumption patterns in app code. Written to re/lix_feature_flags.md.
<!-- SECTION:FINAL_SUMMARY:END -->
