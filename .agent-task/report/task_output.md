issue_title: "[F12 Remediation] Certify Simulation and Approval Paths"
issue_description: |
  **Title**: [F12 Remediation] Certify Simulation and Approval Paths

  **Problem Statement**:
  Finding F12 in `docs/research/native_migration_and_remediation.md` notes that simulation, unknown provider outcomes, and approval paths can look like completion. While some specific fixes have been applied (e.g., invoice stubs no longer invent completion, and generic status changes cannot manufacture payment), unsupported workflows remain unavailable, and all simulation and approval paths have not been fully certified. We need to implement truthful states and receipts, and verify exact authority, stale approval/revocation, and reconciliation checks on affected paths to fully close F12.

  **Research Report**:
  - F12 Context: Simulation, unknown provider outcome and approval paths can look like completion.
  - Required Fixes: Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths.
  - Current Status: Open. Specific fixes applied but full certification lacking.

  **Design Doc**:
  - The system needs to ensure that simulation paths correctly represent their simulated nature and do not emit events or logs that imply actual completion.
  - Unknown provider outcomes must be properly classified as pending or failed, requiring explicit reconciliation rather than assuming success.
  - Approval paths must verify exact authority and handle stale approvals or revocations properly.

  **Implementation Prompt**:
  - Identify all simulation, provider outcome, and approval paths in the system.
  - Implement checks and balances for simulation paths to clearly mark them as simulations.
  - Update provider outcome handling to correctly reflect unknown statuses and require reconciliation.
  - Implement authority, stale approval, and revocation checks for approval paths.
  - Provide a summary of affected paths and the implemented fixes.

  **Priority**: P1

  **Estimated Scope**: Medium

  **Strategy Admission**:
  - Stable OHC target ID: F12 Remediation.
  - Selected customer/stage: All customers.
  - Evidence level: Documented.
  - Baseline/result metric with denominator: N/A.
  - Dependencies/reuse: Existing simulation, provider, and approval logic.
  - Non-goals: Implementing completely new workflows not related to F12.
  - Authority class: Architect.
  - Cost plan: Standard implementation costs.
  - Happy/failure-path acceptance checks: Verified correct states for simulation, unknown outcomes, and approval paths.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
