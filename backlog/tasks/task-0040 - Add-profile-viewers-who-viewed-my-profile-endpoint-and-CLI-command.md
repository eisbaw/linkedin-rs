---
id: TASK-0040
title: Add profile viewers (who viewed my profile) endpoint and CLI command
status: To Do
assignee: []
created_date: '2026-03-24 13:07'
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
- [ ] #1 get_profile_viewers() method added to LinkedInClient
- [ ] #2 CLI 'profile viewers' subcommand shows viewer list
- [ ] #3 Human-readable output shows name, occupation, view stats
- [ ] #4 Endpoint documented in re/profile_viewers.md
<!-- AC:END -->
