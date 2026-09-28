issue_title: "⚡ Bolt: [blocked no-work finding: F04]"
issue_description: |
  # Mission Queue Protocol: Blocked No-Work Finding

  **Target:** F04 - Missing or inconsistent model usage

  **Problem Statement:**
  The current code does not accurately track or enforce usage models across providers and modes.

  **Research Report:**
  The project requires strict CI targets ("30-minute required CI target"). Unfortunately, running the headless tests or compiling the full stack leads to repeated timeouts inside the agent environment when running commands like `make test-backend`, `cargo test --locked --workspace --exclude app --all-targets` and `cargo test -p omnisolo -p server_harness`. This implies the environment cannot compile the workspace binaries or run complete validation suites within the available limits (often timing out after 400s or resulting in `internal error`). Additionally, there are underlying missing dependency bugs across the web tooling (e.g. Next build fails repeatedly due to missing `next` missing or mismatched dependencies, requiring multiple workarounds).

  Because we cannot establish a passing full CI pipeline in the session, and the system fails tests on clean checkout, the rules constrain us from expanding or rewriting unrelated systems merely to bypass them, nor should we manufacture a pass.

  **Verified trace limitations:**
  - `make test` fails because `next` cannot build.
  - `npm run build:web` fails with `next: not found`. Even when reinstalling, test targets fail in CI limits.
  - `make test-rust`, `make test-backend`, `cargo test` consistently time out after ~400s or fail due to internal session timeout limits when trying to test the Rust components.
  - `make lint-backend` fails initially due to a lint error in `src/server/api/agents/client_intake.rs` but addressing it doesn't resolve the fact that tests still cannot complete.

  **Executed test commands:**
  - `make test` (failed)
  - `make lint` (failed)
  - `make test-backend` (timed out)
  - `make test-rust` (timed out/failed)
  - `cargo test --locked --workspace --exclude app --all-targets` (timed out)
  - `cargo test -p omnisolo -p server_harness` (timed out/internal error)
  - `cargo test -p server_harness --test provider_facade` (failed on unmodified checkout)

  **Loaded Superpowers skills/revision:**
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Loaded: `skills/using-superpowers/SKILL.md`

  Since local database E2E limits and compile timeouts prevent running the complete validation gates, I am reporting this as a blocked no-work finding.

issue_priority: "High"
issue_category: "Performance"
issue_type: "blocked no-work finding"
issue_label: ""
assignees: []
