issue_title: "💰 Miser: [blocked no-work finding: F14: economics/owner outcomes]"
issue_description: |
  **Title**: Blocked No-Work Finding: Economics / Owner Outcomes Data (F14)

  **Problem Statement**:
  The codebase lacks measured, representative serving cost data, explicit owner usage distribution data, and reconciled provider invoices required to accurately build the cost/billing models outlined in the business capabilities audit (F14). We cannot define or test proper tiered billing constraints, AI allowances, or budget reservations without this fundamental evidence base.

  **Research Report**:
  - Investigated current telemetry and billing functionality. While paths such as `auditor.rs` and `services/billing/service.rs` provide telemetry and usage accounting, they do not present empirical data on real-world cost or actual customer usage habits across varied workloads.
  - The capability audit (docs/research/business_capability_and_usage_economics_audit.md) explicitly warns against using historical budgets ("invented monthly budgets") to establish token or compute pricing.
  - F14 is marked as blocked due to missing owner economics data and prerequisites.

  **Design Doc**:
  - No system changes proposed.
  - Future implementation will require:
    - Sourcing actual telemetry data from a production or pilot environment.
    - Sourcing provider invoices for cross-referencing and reconciliation.

  **Implementation Prompt**:
  *Not applicable. Blocked on missing data requirements.*

  **Priority**: P2
  **Estimated Scope**: Blocked

issue_priority: P2
issue_category: research
issue_type: report
issue_label: ["agent-report"]
assignees: []
