issue_title: "Blocked Feature implementation"
issue_description: |
  **Title**: Blocked implementation due to full journey validation failure.

  **Problem Statement**:
  The E2E tests (`make test-e2e`) timeout after ~400 seconds. As a result, full-journey validation and architectural readiness for new billing features cannot be established.

  **Research Report**:
  Reviewing the business capability map and code (e.g. `src/server/api/proposals.rs`) shows that we must prove end-user validation. Since the main e2e tests fail in the environment, we cannot confidently deliver these features without breaking the acceptance criteria.

  **Design Doc**: ""
  **Implementation Prompt**: ""
  **Priority**: ""
  **Estimated Scope**: ""
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
