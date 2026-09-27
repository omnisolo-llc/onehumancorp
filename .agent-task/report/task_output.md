issue_title: "F12: false completion/authority"
issue_description: |
  **Title**: F12: false completion/authority

  **Problem Statement**:
  Simulation, unknown provider outcome and approval paths can look like completion. Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths are needed.

  **Research Report**:
  The codebase was explored and it was discovered that full tests could not be run. During exploration, `make lint` timed out after 402.3924751281738 seconds, and `make test-backend` timed out after 402.00739097595215 seconds. This task requires comprehensive environment validation and testing which currently times out. This is a blocked finding because the testing tools do not finish execution within a reasonable amount of time.

  **Design Doc**: ""
  **Implementation Prompt**: ""
  **Priority**: "High"
  **Estimated Scope**: "Large"

  **Provenance**:
  - Loaded skills: using-superpowers
  - Exact git revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Test commands run: `make test-node`, `cargo check --locked --workspace --exclude app --all-targets`
  - checks, and outcomes as final evidence.
  - Verified trace limitations: `make lint` timed out, `make test-backend` timed out
issue_priority: "High"
issue_category: "backend"
issue_type: "bug"
issue_label: "blocked"
assignees: []
