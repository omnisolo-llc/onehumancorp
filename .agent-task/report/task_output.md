issue_title: "💰 Miser: [blocked no-work finding: F05: telemetry not invoice-grade]"
issue_description: |
  **Title:** F05: telemetry not invoice-grade

  **Problem Statement:**
  Current telemetry/cost reports are not an invoice-grade meter. The system needs durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit.

  **Research Report:**
  As per the business capability and usage economics audit (`docs/research/business_capability_and_usage_economics_audit.md`) and the remediation ledger (`docs/research/native_migration_and_remediation.md`), finding F05 is logged as Blocked. The platform compute/idle/support allocation, provider-invoice reconciliation, and payment collection are not completed. Customer payment collection remains disabled on the new usage API. This represents a blocked/no-work outcome due to missing baselines and unverified cost accounting prerequisites. We must measure representative serving costs, establish an internal cost model, and properly attribute usage to payer/auth before implementing this feature. The current OHC contract directive prohibits fabricating data to fill an issue queue.

  **Design Doc:**
  N/A - Blocked finding
  - Mermaid.js architecture diagram: N/A
  - UI wireframes: N/A
  - Mobile UX flow: N/A
  - AI agent integration points: N/A

  **Implementation Prompt:**
  N/A - Verified blocked.

  **Priority:** P2

  **Estimated Scope:** Blocked
issue_priority: "P2"
issue_category: "research"
issue_type: "blocked"
issue_label: "F05"
assignees: []
