issue_title: "Blocked: Google Workspace OAuth lifecycle audit"
issue_description: |
  **Date:** 2026-09-19
  **Scope:** Audit of the Google Workspace OAuth lifecycle, secure credential lifecycle, refresh/revocation, and complete UI-to-provider flows.

  **Finding:**
  The audit of the Google Workspace OAuth lifecycle and wider connector support (identified in F06 of the remediation ledger) is blocked.

  **Reason:**
  A missing provider sandbox and valid test credentials for Google Workspace prevents end-to-end verification. Live credentials cannot be tested, and a dedicated test sandbox setup is an outstanding verification dependency. Without this, we cannot verify normal navigation, result/pending/error states for the connector flow.

  **Uncertainties:**
  - Behavior of the complete UI-to-provider flow for OAuth connection.
  - State handling during refresh and revocation in real scenarios.

  **Action Taken:**
  Recorded as a justified no-work/blocked finding.
issue_priority: "P2"
issue_category: "Integration"
issue_type: "Audit"
issue_label: "blocked"
assignees: []
