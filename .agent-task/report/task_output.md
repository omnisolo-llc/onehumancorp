issue_title: F05 Telemetry not invoice-grade
issue_description: |
  **Title**: F05 Telemetry not invoice-grade

  **Problem Statement**: Current telemetry/cost reports are not an invoice-grade meter. Need durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation, and no duplicate BYOK debit. Platform compute/idle/support allocation, provider-invoice reconciliation, and payment collection are not completed by this ledger.

  **Research Report**: I investigated the requirements for F05: telemetry not invoice-grade. Based on the documentation (`RESEARCH.md`, `business_capability_and_usage_economics_audit.md`, `native_migration_and_remediation.md`), establishing a trustworthy metered billing system requires measuring actual serving cost per eligible attempt, reconciling provider receipts, handling idempotency, rate attribution, payer attribution, tenant isolation, integer subunits for currency/tokens, and more.
  We lack prerequisite capabilities and the data (economic, real workloads, measured outcomes, unit economics) to implement an invoice-grade meter.
  A proposed internal cost model and minimum evidence required are detailed in `docs/research/business_capability_and_usage_economics_audit.md`. The document also states: "Current decision: repair the evidence foundation and evaluate resource-based charging/customer-funded inference. Retain existing business modules; do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research." The status in `docs/research/native_migration_and_remediation.md` is already listed as `Blocked`.

  **Design Doc**: Not applicable - No work required due to missing prerequisites.

  **Implementation Prompt**: The task is blocked. The prompt correctly identifies the role as "Principal Cost Engineer & Miser (L7)" and asks to "recheck current attribution...". However, it specifies that "This mission overrides older role-body examples" and "A no-work/blocked result with evidence is valid".
  Therefore, no implementation changes are required.

  **Priority**: P0
  **Estimated Scope**: Blocked

  **Superpowers Provenance**:
  Loaded Superpowers workflow (`using-superpowers`, `brainstorming`, `writing-plans`).
  Skills revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  No material adaptations.
issue_priority: P0
issue_category: Billing
issue_type: Blocked
issue_label: F05
assignees: []
