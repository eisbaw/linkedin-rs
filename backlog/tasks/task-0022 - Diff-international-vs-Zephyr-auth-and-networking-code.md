---
id: TASK-0022
title: Diff international vs Zephyr auth and networking code
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 07:02'
updated_date: '2026-03-24 07:26'
labels: []
dependencies:
  - TASK-0021
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Compare the auth flow, Routes.java, networking stack, and required headers between the international Voyager and Zephyr China builds. Document any differences that affect the Rust client implementation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Routes.java compared — endpoint differences documented
- [x] #2 Auth flow compared — login endpoint differences documented
- [x] #3 Header requirements compared
- [x] #4 Findings written to re/intl_vs_zephyr_diff.md
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed full diff analysis. Wrote re/intl_vs_zephyr_diff.md covering: Routes.java (376 vs 386 entries, /zephyr/api/ vs /voyager/api/), auth flow (identical POST /uas/authenticate, intl adds passkeys), headers (identical set), Pegasus models (intl has Dash generation, no zephyr namespace), base URLs (same www.linkedin.com).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Produced re/intl_vs_zephyr_diff.md with comprehensive comparison of both decompiled codebases across 5 areas: (1) Routes - intl uses /voyager/api/, Zephyr uses /zephyr/api/; intl has 219 Dash-generation routes + GraphQL endpoint; Zephyr has 126 China-specific routes. (2) Auth - identical POST /uas/authenticate flow, same CSRF generation; intl adds passkey/WebAuthn support; Zephyr adds Flash ID carrier auth. (3) Headers - identical required header set across both builds. (4) Models - intl has no zephyr Pegasus namespace; intl is leaner with Dash migration underway. (5) Base URLs - both default to https://www.linkedin.com. Recommendation: target international build with /voyager/api/ prefix.
<!-- SECTION:FINAL_SUMMARY:END -->
