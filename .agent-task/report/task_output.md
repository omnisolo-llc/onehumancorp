issue_title: "🎥 Lens Audit: F14 No measured representative serving costs or owner outcomes"
issue_description: |
  # Regression Audit Report & Remediation for Translucent Glass UI (Phase 1)

  **Problem Statement:**
  An audit revealed that the "Translucent Glass UI" aesthetic (specifically the `backdrop-filter: blur(30px) saturate(210%)` property) was present in numerous components across the application. However, a significant gap was observed: many Critical User Journeys (CUJs) and E2E tests checking for this UI aesthetic were either missing, incomplete, or bypassed. The audit explicitly mandates:
  * "Every PR must include screenshots proving visual restoration AND logs/test outputs proving data successfully round-tripped from the UI to the DB and back."
  * "E2E Tests (MANDATORY): For every regression fixed, write or update AT LEAST FIVE Playwright E2E tests and/or Web/Desktop/Mobile UI tests that lock in the intended behavior to prevent future drift. The test MUST start from the home page, click through the UI naturally, and assert the database state."

  Currently, I am blocked from executing the full `make test` pipeline successfully due to a missing Docker container (`ohc-e2e-pg`) blocking `pgvector/pgvector` database initialization which is required for Playwright E2E tests, resulting in the failure of `npm run test:e2e`.

  **Research Report (Blocked Prerequisite Details):**
  - **Missing Prerequisites:**
    - The E2E tests require a local instance of PostgreSQL (specifically `pgvector/pgvector:pg15`) to spin up for DB-related assertions, but pulling the Docker image fails consistently due to an extraction error (`failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`).
    - Without the database, the backend (`/app/target/debug/server`) cannot function properly during Playwright tests.
    - Thus, the mandate to "verify the full data lifecycle actually works in reality (UI -> DB -> UI)" is fundamentally blocked.
  - **Superpowers Provenance:**
    - `using-superpowers` skill was loaded from commit `8ca22dba9a94f28898bbce59f2537ff4d87c747d` in `.scratch/superpowers`.
  - **Verified Findings:**
    - `src/ui/next/src/e2e/test_glassmorphism.mock-contract.ts` and `src/ui/next/src/e2e/translucent_glass_ui.mock-contract.ts` exist and assert the styling rules, but they are labeled `.mock-contract.ts` and not integrated properly into the actual DB-verified workflow requirement.
    - `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md` reference F14: "No measured representative serving costs or owner outcomes... Blocked / No-Work due to missing prerequisites and owner economic/metric data."
    - Currently, `npm run build:web` succeeds after installing `lucide-react`, and the backend compiles (`cargo build --locked ...`). However, `npm run test:e2e` fails because Docker cannot pull the required database image.

  **Conclusion:**
  Due to the Docker environment limitation, no complete code change can be safely validated and submitted per the strict testing requirements (no bypasses, no skipped tests). This report serves as the blocked/no-work finding for F14 as it pertains to full-journey testing and UI validation until the infrastructure issue is resolved.

issue_priority: "P0"
issue_category: "ui"
issue_type: "bug"
issue_label: "ohc:journey:J1"
assignees: []
