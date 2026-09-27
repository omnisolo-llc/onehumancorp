issue_title: "F11: Full-journey UI tests timeouts and blockages"
issue_description: |
  **Target ID**: F11

  **Persona/Journey**: Full UI/DB Data Lifecycle Audit

  **Observed versus Inferred Gap**: The full-journey audit requires verifying that UI actions map properly to the DB and reflect back on the screen. However, `make test-e2e` times out.

  **Source/Code Evidence**:
  - `make test-e2e` timed out after 401.92 seconds.

  **Current Behavior**: E2E tests time out before generating verifiable state changes and test results, preventing full-journey data roundtrip audits.

  **Expected Business Result**: Ability to run the full E2E test suite locally and in CI to audit UI features against the DB accurately.

  **Scope/Non-Goals**: Diagnosing or fixing the docker/pgvector environment is out of scope for the Lens UI auditor role.

  **Dependencies**: Fixing E2E execution timeouts and Docker pgvector image extractions in the test environment.

  **Stable Acceptance Criteria**: `make test-e2e` completes successfully without timeouts.

  **Recovery/Authority/Cost Requirements**: N/A for this infrastructure blockage.

  **Bazel Verification**: N/A

  **Report Details**:
  - Trace limitations verified.
  - `make test-e2e` failed due to execution timeout (401s+).
  - Loaded Superpowers Skills: `superpowers:using-superpowers` (read from `.scratch/superpowers/skills/using-superpowers/SKILL.md` at revision f465222936f3b4e1627e51b4c3bd64f901423b63).
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
