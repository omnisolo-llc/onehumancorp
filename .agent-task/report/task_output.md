issue_title: Blocked: E2E and Backend Tests Failing Due to Docker Extraction Error
issue_description: |
  **Title**: Blocked: E2E Tests Failing Due to Docker Extraction Error

  **Problem Statement**:
  The current attempt to run E2E tests using `make test-e2e` and `npm run test:e2e` fails due to an error extracting the PostgreSQL vector database Docker image (`pgvector/pgvector`). Specifically, the error is: `failed to extract layer ... failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`. This prevents the automated E2E and backend tests from running correctly, which is a required step for verification of the Lens audit and full stack journey.

  **Research Report**:
  - Explored the codebase to understand the testing setup (Playwright, Cargo, Makefile).
  - Attempted to build and run native node and cargo projects (`make build-web`, `cargo build`).
  - Tried running `make test-e2e` which timed out.
  - Tried running a specific test using `npm run test:e2e -- src/e2e/login_lens.spec.ts` which successfully built the web assets and rust binaries but failed on initializing the test database because of a Docker layer extraction error: `failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`.
  - Also tried `make test-node` and `cargo check` and `make lint-rust`, resolving dependencies and running into the same timeout issue with the full test-suite.
  - As per memory instruction, E2E failures related to Docker extraction and file permission errors with the `pgvector` image must be treated as a blocked no-work finding.

  **Design Doc**: ""
  **Implementation Prompt**: ""
  **Priority**: ""
  **Estimated Scope**: ""
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
