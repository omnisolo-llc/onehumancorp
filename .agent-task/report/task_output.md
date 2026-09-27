issue_title: "🛡️ Sentry: [blocked no-work finding: Full-Journey and Chaos Validation Blocked by E2E and Backend Test Failures]"
issue_description: |
  **Loaded Skills:**
  - `superpowers:using-superpowers`
  - `superpowers:systematic-debugging`
  - `superpowers:brainstorming`

  **Discovered Git Revision:** `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20`

  **Executed Tests:**
  - `make test-node`: 100% PASS for `src/ui/next` after installing missing `next` dependency; 100% PASS for Node scripts after installing missing `jsdom` dependency. (Note: tests are skipping Tauri/webview dependency requirements in this test lane, which passed.)
  - `make test-backend`: Blocked by execution timeout. (Over 400s execution limitation.)
  - `make test-e2e`: Blocked by execution timeout. (Over 400s execution limitation.)

  **Verified Trace Limitations:**
  The `make test-backend` and `make test-e2e` processes timed out consistently after ~400 seconds, making it impossible to perform chaos validation or measure P50/P95/P99 latencies as mandated by the Principal Reliability Engineer & Sentry role. There was no visible log output revealing the specific reason for the timeout, rendering local triage impossible without a functional test suite or access to the failing containers.

  Due to these verified trace limitations, the required full E2E journeys and backend stress verifications cannot be completed.

  *checks, and outcomes as final evidence.*
issue_priority: "P0"
issue_category: "Reliability"
issue_type: "Defect"
issue_label: "blocked"
assignees: ""
