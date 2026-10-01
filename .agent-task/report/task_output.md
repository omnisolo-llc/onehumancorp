issue_title: "Research: Documenting truthful cost and authority boundaries for owner outcomes (F05)"
issue_description: |
  # Superpowers provenance
  Loaded skills:
  - superpowers:using-superpowers
  - superpowers:brainstorming
  Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d

  # Context
  Target ID: F05 (Current telemetry/cost reports are not an invoice-grade meter)
  Persona/Journey: Owner/Operator configuring standing authority, viewing truthful outcomes and tracking costs.

  # Gap Analysis
  Observed vs Inferred Gap: The audit states that current telemetry/cost reports are not an invoice-grade meter. The gap is that we cannot document verified business cost reconciliation for the owner if the system cannot reliably meter costs.
  Source/Code Evidence: `docs/research/native_migration_and_remediation.md` F05 status is Blocked. The remediation states that "Integer micro-unit durable records bind tenant/task/attempt/provider/model/payer/rate revision; duplicate provider receipts cannot be charged twice; customer-direct usage is not debited as managed inference".
  Current Behavior: Lack of invoice-grade metering makes it impossible to write accurate plain language documentation explaining AI agent costs and billing to the owner.

  # Expected Outcome
  Expected Business Result: Clear plain-language documentation explaining setup, connected accounts, standing authority, evidence, cost, exceptions, and recovery based on actual verified execution.
  Scope/Non-goals: This is a documentation research task. Non-goal is fixing the backend billing logic.
  Dependencies: Requires F05 backend billing to be unblocked and completed.
  Stable Acceptance Criteria: Scribe documentation accurately reflects truthful costs and explains exact authority and reconciliation to the owner in plain language.
  Recovery/Authority/Cost Requirements: Documentation must clearly detail how the owner manages standing policies, hard budget caps, and cost monitoring.
  Bazel Verification: Native Cargo checks are now used, replacing Bazel.

  This is a no-work finding for documentation since the backend capabilities are blocked.
issue_priority: "P1"
issue_category: "documentation"
issue_type: "research"
issue_label: "agent-report"
assignees: []
