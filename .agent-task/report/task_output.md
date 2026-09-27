issue_title: "🗺️ Guide: [blocked no-work finding: E2E Tests Timeout/Timeout/Infrastructure blockers on Onboarding Flow Validation]"
issue_description: |
  # Research Report

  **Issue Description:** The current mission requires addressing an observed activation blocker, testing onboarding registration state via backend databases, and guaranteeing 100% E2E test coverage across multiple viewports to resolve UI/UX gaps in the Onboarding Flow.

  **Finding Details (Blocked/No-work):**
  We attempted to test the E2E framework execution in preparation for creating verified, tested improvements in the onboarding journeys.
  Running `make test-e2e` systematically failed, resulting in:
  `The command timed out after 401.0129346847534 seconds.`

  This execution timeout blocks any full-journey verifications that depend on the full Dockerized stack (PostgreSQL `pgvector` container instances, Tauri web-view validation, and external database state synchronization). Without a reliable runtime environment that handles database initialization and complete testing lifecycles, no feature work involving registration state/backend verification can be confidently implemented and merged per the rigid constraints.

  **Verified trace limitations:**
  The E2E tests command `make test-e2e` timed out after 401.0129346847534 seconds.
  Additionally, the `RESEARCH.md` and current UX docs cite previous known barriers regarding `pgvector` layer extraction permissions and Next.js legacy route resolution errors. As instructed, no simulated successful tests were recorded.

  **Business Impact:**
  The required full Playwright E2E verifications required for any new UI integration cannot be met without unblocking the core CI/CD validation containers. Work on the frontend onboarding UX cannot proceed until the environment correctly supports multi-device `make test-e2e`.

  **Required Next Actions:**
  1. Investigate and resolve the `make test-e2e` test harness timeouts.
  2. Unblock the test dependency configuration and infrastructure limits causing Docker validation to fail.
  3. Resume onboarding implementation tasks only when `make test-e2e` is consistently stable and provides determinism.

issue_priority: "P0"
issue_category: "Infrastructure / E2E Testing"
issue_type: "Defect"
issue_label: "ohc:lane:reliability"
assignees: []
