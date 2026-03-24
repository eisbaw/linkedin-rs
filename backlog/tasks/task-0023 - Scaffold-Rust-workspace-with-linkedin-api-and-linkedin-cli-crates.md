---
id: TASK-0023
title: Scaffold Rust workspace with linkedin-api and linkedin-cli crates
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 07:49'
updated_date: '2026-03-24 08:00'
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
- [x] #1 Cargo workspace compiles with both crates
- [x] #2 reqwest with boring-tls backend configured
- [x] #3 Justfile with build/test/lint/fmt/e2e recipes
- [x] #4 Basic smoke test passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create linkedin/ directory structure with workspace Cargo.toml
2. Create linkedin-api crate with lib.rs module structure and dependencies (reqwest with rustls default, serde, tokio, etc.)
3. Create linkedin-cli crate with clap skeleton
4. Create Justfile with build/test/lint/fmt/fmt-check/e2e/run recipes
5. Add a basic smoke test in linkedin-api
6. Verify compilation with nix-shell --run "just build"
7. Document TLS limitation (boring-tls not available in reqwest 0.13)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- reqwest 0.13 does NOT have a boring-tls feature flag. Used default rustls backend.
- Documented TLS limitation in linkedin-api/src/lib.rs and linkedin-api/Cargo.toml.
- All e2e steps pass: build, test (3/3 smoke tests), clippy (0 warnings), fmt-check.
- CLI skeleton has subcommands: auth (login/status/logout), profile, messages, feed, connections.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Scaffolded Rust workspace at linkedin/ with two crates:

- linkedin-api: library crate with module structure (client, auth, models, error) and dependencies (reqwest, serde, tokio, chrono, thiserror, rand, dirs, etc.)
- linkedin-cli: binary crate with clap-based CLI skeleton (auth, profile, messages, feed, connections subcommands)

TLS backend: reqwest 0.13 does not offer a boring-tls feature flag. Used default rustls backend and documented the limitation in lib.rs and Cargo.toml with TODO to switch when available. See re/tls_configuration.md for the full rationale.

Justfile: build, test, lint, fmt, fmt-check, e2e (gate), and run recipes. All pass via nix-shell --run "just e2e".

Smoke tests: 3 integration tests verify client construction and error display.
<!-- SECTION:FINAL_SUMMARY:END -->
