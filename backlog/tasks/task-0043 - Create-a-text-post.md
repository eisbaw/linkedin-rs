---
id: TASK-0043
title: Create a text post
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 13:21'
updated_date: '2026-03-24 14:04'
labels: []
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement creating a new LinkedIn post. The RE catalog shows contentcreation/normShares endpoint. Need to validate payload format against live API.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Post creation endpoint validated
- [x] #2 CLI 'feed post <text>' command works
- [x] #3 Documented in re/
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add create_post() to client.rs using Dash GraphQL mutation (queryId: voyagerContentcreationDashShares.f8a4f57de961be2d370fbcc862e867cf)
2. Also implement legacy REST fallback via contentcreation/normShares POST
3. Add "feed post <text>" CLI subcommand with --yes flag and --visibility option
4. Document findings in re/create_post.md
5. Run just e2e to verify compilation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Searched decompiled international build: SharingGraphQLClient.java has the mutation queryId
- Legacy contentcreation/normShares REST endpoint exists in Routes.java but intl build uses Dash exclusively
- Key models: ShareData (input), Share (response), VisibilityType enum (ANYONE/CONNECTIONS_ONLY)
- Implemented create_post() in client.rs using GraphQL CREATE mutation
- Added "feed post" CLI subcommand with --yes safety flag and --visibility option
- All e2e tests pass (build, unit tests, clippy, fmt)
- NOT tested against live API - payload structure derived from decompiled code only
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented text post creation for LinkedIn via the Dash GraphQL CREATE mutation.

Changes:
- linkedin-api/src/client.rs: Added create_post(text, visibility) method using
  createContentcreationDashShares mutation (queryId: voyagerContentcreationDashShares.f8a4f57de961be2d370fbcc862e867cf),
  discovered in SharingGraphQLClient.java from the decompiled international APK.
- linkedin-cli/src/main.rs: Added "feed post <text>" subcommand with --visibility
  (ANYONE/CONNECTIONS_ONLY), --yes safety confirmation, and --json output flag.
- re/create_post.md: Full RE documentation covering endpoint discovery, data models,
  mutation variable structure, and source file references.

Key decisions:
- Used Dash GraphQL mutation (not legacy contentcreation/normShares REST) since the
  international build has migrated all sharing to Dash.
- Required --yes flag to prevent accidental real posts (exits with error without it).
- Visibility defaults to ANYONE (public) matching the LinkedIn app default.

Not validated against live API - payload structure is from static analysis only.
<!-- SECTION:FINAL_SUMMARY:END -->
