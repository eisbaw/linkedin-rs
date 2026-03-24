---
id: TASK-0046
title: Job search
status: Done
assignee:
  - '@claude'
created_date: '2026-03-24 13:21'
updated_date: '2026-03-24 14:48'
labels: []
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Extend search to support job listings. The search GraphQL already works for people — add job result type. Check re/search_protocol.md for job-specific filters.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Job search returns results
- [x] #2 CLI 'search jobs <keywords>' command works
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add search_jobs() to linkedin-api client.rs (same as search_people but with resultType:List(JOBS))
2. Add Jobs variant to SearchAction enum in CLI
3. Add cmd_search_jobs() handler with job-specific display (title, company, location)
4. Wire up the dispatch in main match
5. Run e2e tests
6. Live test with cargo run
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- GraphQL searchDashClustersByAll returns HTTP 501 for JOBS resultType -- not supported
- REST search/hits?q=jserpAll also returns 404 (deprecated)
- Found dedicated GraphQL query: jobsDashJobCardsByJobSearch (voyagerJobsDashJobCards.4ef915ad5827cd8ea1351ad72f8e4268)
- Uses JobSearchQueryForInput with origin=FACETED_SEARCH and keywords
- Response shape: elements[].jobCard.jobPostingCard with title/company/location
- Human-readable output shows job title, company, location, and URL path
- Pagination works via start/count params
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented job search via the LinkedIn Voyager GraphQL jobsDashJobCardsByJobSearch query.

Key discovery: The general searchDashClustersByAll GraphQL query (used for people search) returns HTTP 501 for the JOBS result type. The legacy REST search/hits?q=jserpAll endpoint also returns 404. Job search requires the dedicated voyagerJobsDashJobCards GraphQL query, found in CareersGraphQLClient.jobCardsByJobSearch() from the decompiled APK.

Changes:
- linkedin-api: Added search_jobs() using GraphQL query voyagerJobsDashJobCards.4ef915ad5827cd8ea1351ad72f8e4268 with JobSearchQueryForInput (origin=FACETED_SEARCH, keywords, count, start)
- linkedin-cli: Added "search jobs <keywords>" subcommand with --count, --start, --json flags
- Human-readable output: job title, company name, location, job URL path
- JSON output: full GraphQL response for programmatic use

Tests:
- just e2e passes (build, test, lint, fmt-check)
- Live tested: job search returns results, pagination works, people search regression verified
<!-- SECTION:FINAL_SUMMARY:END -->
