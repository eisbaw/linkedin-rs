---
id: TASK-0020
title: Re-download international LinkedIn APK variant
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 06:29'
updated_date: '2026-03-24 07:01'
labels: []
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Current APK is the Zephyr (China) variant. Download the international com.linkedin.android from a different source or region to compare API endpoints, base URLs, and auth flow differences. Use apkeep with different download sources.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 International APK obtained (or confirmed identical to Zephyr)
- [x] #2 Key differences documented if any
- [x] #3 Findings written to re/apk_variant_comparison.md
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Downloaded international Voyager variant (4.1.1183) from APKPure as XAPK. Confirmed Zephyr and Voyager are fundamentally different builds sharing the same package name. International APK saved as linkedin_intl_4.1.1183.apk. Full comparison written to re/apk_variant_comparison.md.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Successfully obtained the international LinkedIn Voyager APK (v4.1.1183, 93.8MB base) from APKPure alongside the existing Zephyr/China variant (v6.1.1, 54.9MB). These are fundamentally different builds: Zephyr uses /zephyr/api/ endpoints, includes Huawei HMS/WeChat/China Unicom integrations, has a React Native layer for social hiring, and targets SDK 29. Voyager uses /voyager/api/ endpoints, includes Google Play/Facebook/Samsung integrations, has no RN bundle, and targets SDK 35. APKPure currently serves Zephyr as 'latest' -- international versions are only available as specific 4.1.xxxx version downloads.
<!-- SECTION:FINAL_SUMMARY:END -->
