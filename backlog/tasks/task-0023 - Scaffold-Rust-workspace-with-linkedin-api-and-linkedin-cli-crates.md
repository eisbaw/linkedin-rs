---
id: TASK-0023
title: Scaffold Rust workspace with linkedin-api and linkedin-cli crates
status: To Do
assignee: []
created_date: '2026-03-24 07:49'
labels: []
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create Cargo workspace at linkedin/ with two members: linkedin-api (library) and linkedin-cli (binary). Set up dependencies per re/tls_configuration.md recommendation (reqwest with boring-tls). Add Justfile recipes for build, test, lint, fmt, e2e.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Cargo workspace compiles with both crates
- [ ] #2 reqwest with boring-tls backend configured
- [ ] #3 Justfile with build/test/lint/fmt/e2e recipes
- [ ] #4 Basic smoke test passes
<!-- AC:END -->
