issue_title: "Blocked: F04 (Missing or inconsistent model usage)"
issue_description: |
  **Target:** F04 (Missing or inconsistent model usage)

  **Status:** Blocked

  **Verified trace limitations:**
  - The goal is to fully remediate F04 ("Missing or inconsistent model usage").
  - `make test-e2e` was executed to verify the full workspace acceptance gates.
  - The `make test-e2e` command timed out after ~402.8 seconds.
  - As established in the system rules, compiling the required Cargo binaries taking longer than the available session limit justifies a blocked no-work finding.
  - "If E2E tests (`make test-e2e` or `npm run test:e2e`) fail or timeout because compiling the required Cargo binaries (e.g., `server`) takes longer than the available session limit (e.g., 400 seconds), this environment limitation justifies a blocked no-work finding."

  **Executed test commands:**
  - `make test-e2e` -> Timed out after 402.8s
  - `cargo test -p server_harness --lib` -> Executed successfully (passed 198 tests in 2m 50s before trace output truncation)

  Since the required workspace E2E testing gate times out before completion, it is impossible to safely test code changes across the full stack as required by the "OneHumanCorp operating contract".
issue_priority: "P0"
issue_category: "infrastructure"
issue_type: "bug"
issue_label: ""
assignees: []
