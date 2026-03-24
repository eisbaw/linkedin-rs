---
id: TASK-0039
title: Implement send message feature
status: To Do
assignee: []
created_date: '2026-03-24 10:48'
labels: []
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add ability to send a message to a LinkedIn connection. Requires finding the correct GraphQL mutation in decompiled/jadx_intl/ for sending messages (likely messengerMessages.create or similar). Implement in linkedin-api client and expose via CLI as 'messages send <recipient> <message>'. This is a write operation — test carefully.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GraphQL mutation for sending messages identified from decompiled code
- [ ] #2 send_message() method added to LinkedInClient
- [ ] #3 CLI 'messages send' subcommand implemented
- [ ] #4 Successfully send a test message to example-user-000000000
<!-- AC:END -->
