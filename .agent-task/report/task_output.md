issue_title: '💰 Miser: F05: economics/owner outcomes'
issue_description: |
  **Title:** Blocked No-Work Finding: F05 - economics/owner outcomes

  **Problem Statement:**
  Audit finding F05 notes that "Current telemetry/cost reports are not an invoice-grade meter". The required remediation is "Durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit". However, as per the `business_capability_and_usage_economics_audit.md` and current state, there is no measured representative serving costs or owner outcomes, making this feature blocked.

  **Research Report:**
  The `docs/research/business_capability_and_usage_economics_audit.md` file notes that cost accounting requires:
  "The current source does not supply a measured deployment cost, representative workload distribution or reconciled provider invoice. Do not replace the former invented monthly budgets with invented per-token or per-compute prices."
  It further specifies: "Capture durable, deduplicated events with tenant/project/task/attempt and provider request IDs; payer and auth mode; provider/model; actual input/output/cache or tool quantities; resource unit and measured interval; rate-card version; estimated/reserved/settled/refunded state; and external reconciliation reference. API balances, subscription quota and customer cash are not interchangeable units."

  Without actual usage economics, rates, and reconciliation evidence, building an invoice-grade telemetry system is blocked. It requires missing data and prerequisites. Therefore, this is a blocked no-work finding.

  **Design Doc:**
  N/A

  **Implementation Prompt:**
  N/A

  **Priority:** P1
  **Estimated Scope:** Blocked
issue_priority: P1
issue_category: reliability
issue_type: report
issue_label: agent-report
assignees: []
