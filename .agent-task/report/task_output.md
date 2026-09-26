issue_title: "F14: economics/owner outcomes"
issue_priority: "P1"
issue_category: "RESEARCH"
issue_type: "RESEARCH"
issue_label: "ohc:lane:finance"
assignees: []
issue_description: |
  # Issue: No measured representative serving costs or owner outcomes

  ## Problem Statement
  The usage economics audit identified issue F14: "No measured representative serving costs or owner outcomes". The codebase has workload usage records and build/resource timing available, but there are no measured representative customer-serving costs, owner interviews, willingness-to-pay results, or paid-retention results measured in this work.

  ## Research Report
  The current source does not supply a measured deployment cost, representative workload distribution, or reconciled provider invoice. Before proposing concrete fixes for performance, serving costs, or API charging, the missing data (owner interviews, baseline economics) must be gathered. Wait times and infrastructure resource demands cannot be fully modeled without understanding actual workflows.

  ## Superpowers Workflow Provenance
  - Loaded skills: `using-superpowers`, `brainstorming`, `systematic-debugging`, `writing-plans`, `executing-plans`, `verification-before-completion`.
  - Repository URL: https://github.com/obra/superpowers/
  - Revision hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks performed:
    - Verified `MAX_DB_RETRY_ATTEMPTS` in `src/server/db.rs`.
    - Evaluated `execute_with_retry` and related integration tests (`src/server/orchestration/state/parity_test.rs`).
    - Attempted to run headless integration suites but encountered 400s timeouts.
    - Inspected `F14` findings in the `docs/research/native_migration_and_remediation.md` ledger.
  - Outcomes: The attempt to optimize performance and set budgets is **no-work/blocked**.

  ## Blocked Prerequisites
  1. We must conduct owner interviews to map real business workflows and baseline alternatives.
  2. We must measure a representative workload using the existing metrics infrastructure to establish baseline CPU/memory, wait times, and API token counts.
  3. We must not manufacture an arbitrary price list or margins without usage logs and reconciliation tests in a customer environment.

  ## Estimated Scope
  Blocker resolution requires non-engineering tasks (customer interviews, analytics review). No code changes should be proposed until the data is provided.
