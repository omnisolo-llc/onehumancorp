issue_title: "💰 Miser: [blocked no-work finding: F05]"
issue_description: |
  # Research Report: F05 - Cost Accounting Before a Price Card

  **Target Evaluated:** F05 (Current telemetry/cost reports are not an invoice-grade meter)

  **Current Status Evaluated:** Blocked / Missing prerequisites.

  ## Owner Economics & Capability Requirements
  According to `docs/research/business_capability_and_usage_economics_audit.md`, implementing invoice-grade telemetry requires durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit.

  Furthermore, before implementing a concrete price card, the following evidence is required:
  - Can the same authorized workload be costed in managed API and customer-funded modes?
  - For each representative workload measure owner setup/review/correction time; observed success/failure; provider requests and token classes; active/reserved resources; cold starts, waits and retries; stored/network data; support effort; and an actual invoice reconciliation.
  - Report workload size, deployment mode, payer, sample count and p50/p95 rather than extrapolating from one demo.

  ## Source State
  - The current event accounting pipeline (`src/server/services/billing/auditor.rs`) has fields for caching, tokens, compute, etc.
  - However, the `UsageLedger` and `PayerMode` concepts are mostly defined in `src/server/harness/middleware/usage_ledger.rs` and are not fully reconciled across the backend.
  - Implementing this requires an authorized owner economics and workload study that is currently absent. "Research evidence alone does not certify these gates or a shipped business outcome."

  ## Conclusion
  The ticket F05 requires missing evidence and prerequisites defined in `business_capability_and_usage_economics_audit.md`. According to the instructions, "a missing prerequisite is an outstanding verification dependency, not a pass" and "a no-work/blocked result with evidence is valid".

  This is submitted as a blocked no-work finding.
issue_priority: "P2"
issue_category: "Costing"
issue_type: "Research"
issue_label: "blocked"
assignees: []
