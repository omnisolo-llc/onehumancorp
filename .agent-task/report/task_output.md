issue_title: "F11: Named full-journey tests only delegate to a smoke helper"
issue_description: |
  F11 requires adding actual mutation/state/provider-boundary acceptance tests without live credentials.
  However, this task requires full-journey validations via real stack components, including E2E.
  During execution, attempting to run `npm run test:e2e` repeatedly timed out (first after 401.13 seconds, then `make build-e2e && npm run test:e2e` timed out after 401.35 seconds).
  Due to the test environment being unable to complete the E2E tests within the timeout, this task is a blocked no-work finding.

  **Skill Provenance and Final Evidence**
  - Loaded skills: `using-superpowers`
  - Exact git revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Executed test commands:
    - `npm run test:e2e` (Timed out after 401s)
    - `make test-e2e` (Timed out after 401s)
    - `make lint-node`
    - `make test-node`
    - `cargo check --locked --workspace --exclude app --all-targets`
  - checks, and outcomes as final evidence: tests ran to failure (timeout), blocking F11 E2E tests validations.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
