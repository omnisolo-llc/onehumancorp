issue_title: "🎥 Lens Audit: [no-work finding]"
issue_description: |
  # Audit Report: F11 Full-Journey Tests & Build Dependencies

  ## Selected Issue
  F11 from docs/research/native_migration_and_remediation.md: "Named full-journey tests only delegate to a smoke helper" (Blocked).

  ## Investigation & Evidence
  - `src/e2e/full_journey_e2e.spec.ts` was inspected and it confirms the finding: it only delegates to `currentAppSmoke`.
  - Attempted to run the E2E suite (`npm run test:e2e -- src/e2e/lens_audit.spec.ts`) to audit the actual UI against the Lens standards (8px/16px rounded corners, glass materials, etc.).
  - The local `cargo build` and `cargo test` processes fail due to resource exhaustion/OOM in the sandbox environment ("Internal error occurred when running command").
  - The E2E test runner (`scripts/native-e2e.mjs`) correctly enforces the presence of freshly built native Rust backend binaries (`target/debug/server`) and Next.js standalone web artifacts (`target/native-web`).
  - Because the Rust backend cannot be compiled, the required dependencies for `make test` and full-stack E2E tests are unavailable.
  - According to the instructions, "a missing prerequisite is an outstanding verification dependency, not a pass".

  ## Conclusion
  No UI regressions could be remediated or visually verified due to the blocked local build environment and inability to launch the real-stack E2E test suite. This is a blocked / no-work finding.

  **Provenance**:
  - Loaded skills: `superpowers` (hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - Pre-existing failures: `cargo build` OOM, `make build-web` lacking necessary artifacts in CI.

issue_priority: "P0"
issue_category: "ui"
issue_type: "bug"
issue_label: "ohc:journey:J1"
assignees: ["lens"]
