---
id: TASK-0004
title: Decompile DEX files with jadx
status: Done
assignee:
  - '@claude'
created_date: '2026-03-23 22:32'
updated_date: '2026-03-23 22:45'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Run jadx on all DEX files from the extracted APK. Handle potential memory issues with large LinkedIn APK.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All DEX files decompiled to decompiled/jadx/
- [x] #2 Decompilation errors documented
- [x] #3 jadx output is browseable
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- jadx was not in shell.nix; added `jadx` to buildInputs (nixos.jadx 1.5.0)
- Ran: `jadx -d decompiled/jadx com.linkedin.android.apk` with JADX_OPTS="-Xmx4g"
- Processed 27,332 classes across 5 DEX files
- Completed with 33 errors (0.12% error rate)
- Output: 362MB, 26,062 Java source files
- 32 files contain partial decompilation errors (jadx exception comments)
- Error breakdown: 14 in Google GMS/ads, 6 in LinkedIn code, 4 in LinkedIn xmsg, 2 in Jackson, 2 in ASM, 2 in Huawei/CMIC SDKs, 2 in Google contextmanager
- LinkedIn-affected files: MessagingLegoUtil, InMailResponse, AttachmentFileType, UnfollowHubBundleBuilder, HeathrowSource, xmsg/Type, xmsg/ErrorMessage
- All errors are in non-critical code; core LinkedIn packages (auth, networking, feed, identity, messaging, jobs, search) decompiled cleanly
- decompiled/ is already in .gitignore
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Decompiled LinkedIn Android APK (53MB, 5 DEX files, 27,332 classes) using jadx 1.5.0.

Changes:
- Added `jadx` to shell.nix buildInputs (was missing; jdk was present but jadx itself was not)
- Ran jadx decompilation: 362MB output, 26,062 Java source files in decompiled/jadx/

Results:
- 33 decompilation errors out of 27,332 classes (0.12% error rate)
- 32 files have partial errors: mostly Google GMS/ads internals (14 files), plus 7 LinkedIn files in non-critical areas (messaging utils, xmsg placeholders, growth/heathrow)
- Core LinkedIn packages decompiled cleanly: authenticator, networking, feed, identity, messaging, jobs, search, relationships, liauthlib, pegasus, realtime
- Top-level packages: com.linkedin.android (90+ sub-packages), com.linkedin.avro2pegasus, com.linkedin.data, com.linkedin.gen, com.linkedin.lix, com.linkedin.security, com.linkedin.xmsg
- decompiled/ already gitignored

No regressions. No code changes beyond shell.nix.
<!-- SECTION:FINAL_SUMMARY:END -->
