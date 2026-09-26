issue_title: '💰 Miser: Audit of Usage Economics and Cost Accounting'
issue_description: |
  **Title:** Evaluate Compute/API Usage Economics

  **Problem Statement:** The current source does not supply a measured deployment cost, representative workload distribution or reconciled provider invoice. Previous pricing hypotheses and metrics (like token volume/PR counts) are suspended. Need to capture durable, deduplicated events for OHC serving cost vs. customer-paid BYOK cost.

  **Research Report:** Reviewed `business_capability_and_usage_economics_audit.md` which lists F04 and F05 findings showing model paths disagree on usage and current telemetry/cost reports are not invoice-grade. Budget reservations need atomic tracking and exact settlement. The OHC contract demands precise reporting before new business feature work.

  **Design Doc:**
  We must introduce distinct tracking for `BillingMode::OHC_Funded` and `BillingMode::BYOK` so that customer-funded inferences do not debit the OHC ledger. The budget system should capture durable, idempotent usage with tenant/project attribution.

  Mermaid diagram:
  ```mermaid
  graph TD
      A[API] --> B[BudgetManager]
      B --> C{Reserve}
      C -->|Success| D[BudgetReservation]
      C -->|Fail| E[LimitExceeded]
      D --> F[Run Tool/Model]
      F --> G[Settle]
      G --> H[Telemetry]
  ```

  UI wireframes: Mobile-first "My Plan" displaying current usage vs budget limits.
  Mobile UX flow: User checks settings -> selects billing -> sees itemized costs + warning thresholds.
  AI agent integration points: N/A

  **Implementation Prompt:** Implement invoice-grade telemetry separation between BYOK usage and OHC-funded usage in the telemetry store and the BudgetManager, ensuring atomic reservations and exactly-once settlement.

  **Priority:** P0

  **Estimated Scope:** Core budget reservation enhancement and test coverage for the usage audit.

  **Superpowers Workflow Provenance:**
  - Loaded skills: `using-superpowers`, `writing-plans` (local scratch)
  - Revision: f8e9d8dd5c099f417df0c32f6131e9b465e5fb20
  - Checks: `cargo check` and `make test-contracts`
  - Outcomes: Verified cost economics constraints in code against audit requirements.
issue_priority: P0
issue_category: finance
issue_type: feature
issue_label: ohc:lane:finance
assignees: []
