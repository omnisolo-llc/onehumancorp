outcome: no_work
issue_title: Universal Provider & Local Usage Accounting (F04)
issue_description: |
  **Outcome**: no_work

  **Verification Evidence**:
  - `src/server/api/proposals.rs` was checked, and it properly uses `reason_with_usage` to extract token usage (input, output, and cached_input) from the local LLM client via `ResearcherLlmClient`, and no dummy hardcoded tokens are injected into `CostAuditor`.
  - `src/server/harness/middleware/provider_facade.rs` was verified to bind `include_usage` natively into streaming requests and streams are tracked via `UsageCapture`, sending the accumulated provider token counts dynamically to the ledger.
  - `src/server/local_generation.rs` correctly translates raw token counts (`prompt_eval_count`, `eval_count`, `prompt_eval_cached_count`) from actual usage arrays.

  No further active modifications were required to satisfy F04, as the implementation requested in the issue has already been completed in this branch.

  **Superpowers Checklist**:
  Loaded SKILL.md paths:
  - `skills/using-superpowers/SKILL.md` (Commit: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)

  Execution outcomes: Confirmed no_work status.
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
issue_priority: P0
