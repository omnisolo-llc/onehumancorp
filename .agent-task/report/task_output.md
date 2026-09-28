issue_title: "🎥 Lens Audit: [blocked no-work finding: F11]"
issue_description: |
  # Audit Report

  - Target: F11
  - Status: Blocked No-Work Finding
  - Justification: The local E2E environment is fundamentally broken for local Playwright tests (`npm run test:e2e`).

  Verified trace limitations:
  - Playwright test runner failed due to a missing dependency `@playwright/test/cli` (which had to be manually installed).
  - Web asset build failed due to missing `lucide-react` (which had to be manually installed).
  - The E2E tests then still fail with `[native-e2e] Required native test input missing: /app/target/debug/server. Build Cargo binaries and run npm run build:web first.`
  - Compiling `server` takes longer than the available timeout limit (400 seconds limit). `cargo build --locked --release -p omnisolo --bin server` failed.

  Therefore, this job is unable to start up the environment to do the UI/DB testing necessary to fulfill the "Lens Audit" role instructions. No actual code modifications were made.
issue_priority: P0
issue_category: ui
issue_type: bug
issue_label: agent-report
assignees: []
