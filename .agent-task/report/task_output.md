issue_title: "F11: Named full-journey tests only delegate to a smoke helper"
issue_description: |
  **Title**: F11: Named full-journey tests only delegate to a smoke helper

  **Problem Statement**:
  F11 requires adding actual mutation/state/provider-boundary acceptance tests without live credentials. However, running `make test-e2e` resulted in an execution timeout after 401.08 seconds, blocking database/full-journey validations.

  **Research Report**:
  This task targeted the repair of F11 from the remediation ledger. Execution of `make test-e2e` timed out after 401.08 seconds. Furthermore, execution of `cargo check --locked --workspace --exclude app --all-targets` timed out after 401.11 seconds. `make test-backend` failed with an internal error. Due to these verified trace limitations, this is a blocked no-work finding. No implementation of F11 can be properly verified.

  **Design Doc**: ""
  **Implementation Prompt**: ""
  **Priority**: ""
  **Estimated Scope**: ""
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
