---
id: TASK-0033
title: Document all model mismatches found during live validation
status: To Do
assignee: []
created_date: '2026-03-24 07:50'
labels: []
dependencies:
  - TASK-0027
  - TASK-0028
  - TASK-0029
  - TASK-0030
  - TASK-0031
  - TASK-0032
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Consolidate all differences between decompiled models and actual API responses discovered during TASK-0026 through TASK-0032. Update re/pegasus_models.md or create re/model_corrections.md with the ground truth from live API.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All model mismatches cataloged
- [ ] #2 Corrected field types/names documented
- [ ] #3 Missing or extra fields documented
- [ ] #4 Recommendations for Rust serde structs written
<!-- AC:END -->
