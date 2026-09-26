issue_title: "💰 Miser: Evaluate compute/API charging and BYOK mode"
issue_priority: "P0"
issue_category: "reliability"
issue_type: "feature"
issue_label: "ohc:lane:finance"
assignees: []
issue_description: |
  **Title**: 💰 Miser: Evaluate compute/API charging and BYOK mode

  **Problem Statement**:
  Evaluate charging for compute and AI API usage, plus customer BYOK or provider-permitted native subscription access, and understand existing code and real owner needs before committing to another plan. As stated in the guidelines: "A no-work/blocked result with evidence is valid. New epics need an explicit evidence-backed decision".

  **Research Report**:
  - The audit (`docs/research/business_capability_and_usage_economics_audit.md`) finds that:
    "Current decision: repair the evidence foundation and evaluate resource-based charging/customer-funded inference. Retain existing business modules; do not build another generic assistant... Keep proposal, launch, retail and field-service paths as candidates until code verification and owner evidence justify selection."
  - Furthermore, "The earlier $99 subscription, 300-step allowance, $299 setup, fixed cohort/margin targets and exclusive web/design/marketing segment are suspended hypotheses, not accepted requirements."
  - An investigation of `src/server/pricing/budget.rs` shows that the `C. A budget monitor is not a hard spending reservation` issue has been successfully repaired and atomic reserve/settle is active. This is backed by `docs/research/native_migration_and_remediation.md` ("pricing/budget.rs:49-84 increments before reporting over-limit | Atomic reservation before spend; settle/release/replay/restart/concurrency checks; invalid/overflow amounts fail closed | Closed").
  - `src/server/pricing/miser_verified.txt` shows that all Miser components are present.
  - Since the defect is repaired and we are blocked from selecting arbitrary rates or implementing generic pricing without evidence-backed workflows from real owner usage, this task results in a blocked/no-work state.

  **Design Doc**:
  ```mermaid
  flowchart LR
      O[No Work] --> L[Evidence and Code Verification Blockers]
  ```

  **Implementation Prompt**:
  N/A

  **Priority**: P0
  **Estimated Scope**: No-work

  **Blocked Prerequisites**:
  Feature expansion for pricing/billing is blocked. We need measured workload distributions, reconciled provider invoices, and a small, permissioned set of recent owner workflows across candidate segments before choosing a segment or setting prices.

  **Superpowers Provenance**:
  - `using-superpowers` (8ca22dba9a94f28898bbce59f2537ff4d87c747d) loaded.
  - `brainstorming` loaded to explore the codebase and determine that this is blocked.
