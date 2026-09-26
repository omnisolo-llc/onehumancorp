# F14 Blocker: Missing Docker Environment for E2E Tests

**Finding:** F14: No measured representative serving costs or owner outcomes
**State:** Blocked

## Blocker Description
The implementation of the F14 finding, which mandates E2E verification of serving costs and visual regressions, is currently blocked. The Playwright E2E tests require a local instance of PostgreSQL (specifically `pgvector/pgvector:pg15`) to spin up for database-related assertions. However, pulling the Docker image fails consistently due to an extraction error (`failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`) in the execution environment.

Without the database, the backend (`/app/target/debug/server`) cannot function properly during Playwright tests, making it impossible to "verify the full data lifecycle actually works in reality (UI -> DB -> UI)" as required by the audit.

## Unblocking Criteria
1. Provisioning a fully functional Docker environment or container runtime in the test/execution environment.
2. Successfully pulling and running the required `pgvector/pgvector:pg15` image (e.g., via the `ohc-e2e-pg` container).
3. Confirming that `npm run test:e2e` can execute without infrastructure-related failures so that representative serving costs and visual regressions can be measured under realistic workloads.
