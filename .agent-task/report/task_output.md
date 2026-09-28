issue_title: "🗺️ Guide: [blocked no-work finding: F11]"
issue_description: |
  Target: F11
  Problem Statement:
  The codebase is currently missing actual mutation/state/provider-boundary acceptance tests without live credentials. Named full-journey tests only delegate to a smoke helper.

  Verified trace limitations:
  - `make test-e2e` fails during the rust cargo build step which times out, thus we cannot fully run the local e2e tests required to verify frontend onboarding flows. E2E tests (`npm run test:e2e`) fail or timeout because compiling the required Cargo binaries (e.g., `server`) takes longer than the available session limit.

  Expected Business Result:
  Due to the limitations in the environment, we cannot execute the required onboarding/ui acceptance testing. This limitation justifies a blocked no-work finding.

  Executed test commands:
  - `make test-e2e > e2e_test.log 2>&1 || npm run test:e2e > e2e_test2.log 2>&1 &` (Timed out/exited early)
issue_priority: "P0"
issue_category: "ui"
issue_type: "blocked"
issue_label: "ohc:lane:ui"
assignees: []
