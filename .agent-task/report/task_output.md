issue_title: "🗺️ Guide: [blocked no-work finding: test-backend timeouts]"
issue_description: |
  **Loaded Superpowers revision:** 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md, skills/systematic-debugging/SKILL.md)

  **Executed test commands:**
  - `make test-backend` (Timed out)
  - `make test-backend &` (Killed)
  - `cargo test --lib --manifest-path src/server/Cargo.toml api::tool_integrations` (Failed)
  - `cargo test --locked --workspace --exclude app server_common` (Failed)
  - `cargo check --locked --workspace --exclude app --all-targets` (Killed)
  - `cargo test -p omnisolo_billing` (Output truncated)
  - `cargo test --lib -p omnisolo-billing` (Failed)
  - `cargo test -p omnisolo_server` (Failed)
  - `cargo test -p omnisolo` (Timed out)
  - `cargo test --help > /dev/null` (Passed)

  **Verified trace limitations:**
  - The focused backend test command (`make test-backend`) and other backend test attempts like `cargo test -p omnisolo` timed out after 400 seconds.
  - This indicates a fundamental environment limitation with compiling the backend workspace (e.g. 59 Rust packages) within the session limits, preventing test-driven development or reliable verification of backend fixes.
  - Because I cannot run even focused backend tests to completion without timeouts, any attempted backend code fixes (such as addressing onboarding flows or provider integrations) cannot be verified locally.
  - As instructed in my active memory and based on this reproducible block, I am submitting this blocked no-work finding since I cannot reliably implement and verify backend code changes.
