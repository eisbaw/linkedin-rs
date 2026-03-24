---
id: TASK-0025
title: Implement manual auth flow (li_at cookie injection)
status: To Do
assignee: []
created_date: '2026-03-24 07:49'
labels: []
dependencies:
  - TASK-0024
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Since automated login triggers CAPTCHA, implement a manual auth mode: user provides li_at cookie value (from browser dev tools), client injects it into the cookie jar alongside a generated JSESSIONID. Persist tokens to ~/.local/share/linkedin/session.json. Per re/auth_flow.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 User can provide li_at cookie via CLI flag or env var
- [ ] #2 JSESSIONID auto-generated and paired with li_at
- [ ] #3 Session persisted to disk
- [ ] #4 Session loaded on subsequent runs
- [ ] #5 Auth status check (call a simple endpoint to verify session works)
<!-- AC:END -->
