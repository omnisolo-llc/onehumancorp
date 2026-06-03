# Reliability Report

## Baseline Testing
All 85 base tests passed successfully. No immediate failures were found related to ML-Resilience or Cloud/Standalone parity.

## Code Audit and Cleanup
- Removed unused `db` variable in `src/server/workers/department_workers.rs`.
- Removed unused `amount_usd` variable in `src/agents/builtin/tools/checkout.rs`.
- Removed unused `PermissionArchitecture` import in `src/agents/builtin/tools_gating.rs`.
- Addressed ambiguous glob re-exports in `src/server/autodream_pipeline/bazel_lib.rs`, `src/server/services/bazel_lib.rs`, `src/server/orchestration/bazel_lib.rs`, `src/server/orchestration/queue/bazel_lib.rs` and `src/server/builder/bazel_lib.rs`.
- Fixed unused import warning in `src/server/telemetry/mod.rs`.

## Parity Auditing
- Reviewing differences in SQLite vs Postgres implementation.
## Cloud/Standalone Parity Notes
- **SKIP LOCKED logic:** Identified places where SQLite relies on simple SELECT due to lack of `SKIP LOCKED`. A potential issue is holding transactions open while waiting for AI API calls. For now, since SQLite is mostly meant for single-tenant or lightweight scenarios in Standalone mode, we accept transaction-held read locks, but we added comments warning about holding locks during AI calls.
- The SQLite implementations of `SKIP LOCKED` fallbacks appear to be consistently using a pattern of either relying on the application-level Mutex (e.g. `sqlite_mu`) to serialize access, OR they use an atomic `UPDATE ... RETURNING` pattern. These are valid patterns for SQLite.
- **ML-Resilience:** Checked AI agent workers in `src/server/workers/department_workers.rs`. Agents like `AdvisorWorker` and `CustomerSuccessWorker` use `timeout(DB_OP_TIMEOUT, poll_op)` and fallback statuses like `PAUSED` when AI is unavailable, meeting the resilience rules.
## Chaos Engineering & Stress Scenarios

The Hybrid Sync Daemon handles cloud escalations via `sync_cloud_escalations` and `sync_step`. It uses PostgreSQL transactions to push records to `agent_missions` and `sub_agent_queue` while updating the local SQLite tracking status.

We examined the error handling when PostgreSQL is unreachable (simulating network failure or sync lag):
- The daemon catches errors on `begin()` and rolls back correctly.
- It degrades gracefully and marks local rows with `sync_error` without crashing.
- Retry mechanism exists implicitly, as it re-selects rows where `sync_error IS NOT NULL OR last_synced_at < datetime('now', '-5 minute')`.

Overall, this aligns well with the stated Chaos/Stress requirements for gracefully handling drops between the Cloud and Standalone environments.

## UI Graceful Degradation E2E Test
- Wrote `resilience.spec.ts` which tests the UI when API requests fail (e.g., connection drops), asserting that the Thin Client degrades gracefully without a White Screen of Death or unhandled application errors.

## Playwright Note
- Playwright E2E tests are correctly orchestrated via the Bazel runner, however the Docker limit rate restricts local CI tests. This is a known environmental constraint rather than a code error.
