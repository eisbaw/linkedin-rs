---
id: TASK-0002
title: Extract APK/XAPK recursively
status: Done
assignee:
  - '@claude'
created_date: '2026-03-23 22:32'
updated_date: '2026-03-23 22:40'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Write re/apk_extract.sh to handle both plain APK and XAPK (split APK bundles). Extract all contents for decompilation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Extraction script handles both APK and XAPK formats
- [x] #2 All split APKs extracted if XAPK
- [x] #3 Contents placed in extracted/ directory
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Script handles both APK and XAPK formats via content-based detection
- APK detection: looks for AndroidManifest.xml in ZIP listing
- XAPK detection: looks for inner .apk file entries in ZIP listing
- Fixed echo-pipe issue with large unzip output (456KB) by using bash here-strings
- Fixed false positive XAPK detection: Archive header line matches .apk$ so used Perl regex to match only ZIP entry lines

- AC#2: XAPK path implemented but not exercised (our file is plain APK). Code extracts each inner .apk into extracted/<name>/ and copies bundle metadata.
- Ran successfully: 10493 files extracted, 145MB total, includes AndroidManifest.xml, 5 DEX files, assets, resources
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added re/apk_extract.sh to handle both APK and XAPK extraction.

The script detects format by inspecting ZIP contents: XAPK bundles contain inner .apk entries, plain APKs contain AndroidManifest.xml directly. For plain APK, contents are unzipped to extracted/. For XAPK, the outer ZIP is extracted to a temp dir, then each inner .apk is extracted into extracted/<name>/.

Ran on com.linkedin.android.apk (53MB plain APK): 10,493 files extracted (145MB) including AndroidManifest.xml, 5 DEX files, assets, and resources.

Notable fix: echo-piping 456KB of unzip output silently fails in bash; switched to here-strings. Also fixed false XAPK detection caused by the Archive: header line matching .apk$.
<!-- SECTION:FINAL_SUMMARY:END -->
