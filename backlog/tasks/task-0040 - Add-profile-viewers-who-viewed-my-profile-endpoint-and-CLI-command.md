---
id: TASK-0040
title: Add profile viewers (who viewed my profile) endpoint and CLI command
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 13:07'
updated_date: '2026-03-24 13:12'
labels: []
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The legacy endpoint GET /voyager/api/identity/wvmpCards works and returns profile viewer data without Premium. Returns viewer names, occupations, view count change percentage, and aggregated recruiter views. Implement get_profile_viewers() in linkedin-api and 'profile viewers [--json]' subcommand in CLI. Document endpoint in re/.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 get_profile_viewers() method added to LinkedInClient
- [x] #2 CLI 'profile viewers' subcommand shows viewer list
- [x] #3 Human-readable output shows name, occupation, view stats
- [x] #4 Endpoint documented in re/profile_viewers.md
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add get_profile_viewers() to LinkedInClient in client.rs
2. Add Viewers subcommand to ProfileAction in main.rs
3. Implement cmd_profile_viewers() with human-readable parsing of nested Rest.li union
4. Create re/profile_viewers.md endpoint documentation
5. Run e2e tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Added get_profile_viewers() to LinkedInClient (simple GET identity/wvmpCards)
- Added profile viewers [--json] subcommand with parsing for 5 union types:
  WvmpProfileViewCard, WvmpGenericCard, WvmpAnonymousProfileViewCard, PrivateProfileViewer, WvmpPremiumUpsellCard (skipped)
- Created re/profile_viewers.md documenting the endpoint and response structure
- Validated live: shows named viewers with occupation, aggregated recruiter views, anonymous viewer counts
- e2e passes (build, test, lint, fmt-check)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added profile viewers (who viewed my profile) feature.

API:
- get_profile_viewers() in LinkedInClient calls GET /voyager/api/identity/wvmpCards

CLI:
- `profile viewers [--json]` subcommand parses 4-level-deep Rest.li union nesting
- Human-readable output shows view count change %, named viewers with occupation,
  aggregated entries (e.g. recruiters), anonymous viewer counts
- Handles 5 union card types: WvmpProfileViewCard, WvmpGenericCard,
  WvmpAnonymousProfileViewCard, PrivateProfileViewer, WvmpPremiumUpsellCard (skipped)

Docs:
- re/profile_viewers.md documents endpoint, auth requirements, and full response structure

Validated live and via e2e gate (build, test, lint, fmt-check).
<!-- SECTION:FINAL_SUMMARY:END -->
