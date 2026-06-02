# KAIROS Orchestration Queue Parity Audit
**Date:** $(date +%Y-%m-%d)
**Auditor:** Principal Reliability Engineer & Sentry (L7)
**Target:** `sub_agent_queue` vs `ohc_job_queue`

## Executive Summary
This audit evaluates the hybrid parity status of the background task orchestration queues used by KAIROS. The platform currently has two overlapping queue implementations:
1. `sub_agent_queue` (Managed via `QueueManager` in `src/server/queue.rs`)
2. `ohc_job_queue` (Managed via `TaskQueue` trait implementations `PgTaskQueue` and `SQLiteTaskQueue`)

**Finding:** The `sub_agent_queue` implementation severely violates Hybrid Parity requirements as it is strictly coupled to PostgreSQL (`PgPool`). This makes it inherently incompatible with Standalone Mode deployments using SQLite.

## Detailed Analysis

### 1. `sub_agent_queue` (Legacy Implementation)
- **Location:** `src/server/queue.rs` (`QueueManager` struct).
- **Schema:** Defined manually in `db.rs` / `queue.rs` via `CREATE TABLE IF NOT EXISTS` commands instead of controlled database migrations.
- **Parity Gap:** The `QueueManager` struct strictly requires a `sqlx::PgPool` for initialization: `pub fn new(pool: sqlx::PgPool) -> Self`.
- **Standalone Support:** **FAIL**. It is impossible to pass an `SqlitePool` to this manager. Standalone Mode utilizing `sub_agent_queue` will either crash or fail to initialize the Orchestrator queue.
- **Isolation:** Uses raw string concatenation for row-level operations instead of relying on the standard Row Level Security (RLS) policies configured for unified tenant architecture.

### 2. `ohc_job_queue` (Modern Implementation)
- **Location:** `src/server/orchestration/queue/pg_queue.rs` and `src/server/orchestration/queue/sqlite_queue.rs`.
- **Schema:** Properly defined in controlled migrations (e.g., `src/server/db/migrations/015_job_queue_and_ledger.sql`, `060_job_queue_and_ledger.sql`). Includes explicit RLS policies (`tenant_isolation_ohc_job_queue`).
- **Parity Support:** **PASS**. The system abstracts queue operations behind the `TaskQueue` async trait. Both PostgreSQL (`PgTaskQueue`) and SQLite (`SQLiteTaskQueue`) implementations exist and successfully compile.
- **Locking:** SQLite safely implements concurrency via an application-level Mutex (`tokio::sync::Mutex<()>`) while Postgres correctly utilizes `FOR UPDATE SKIP LOCKED`.

### 3. Test Coverage Gaps
- `test_sub_agent_queue_isolation` in `src/server/queue_test.rs` relies on an active `OHC_DATABASE_URL` (assumed PostgreSQL) to pass. It explicitly uses `PgPoolOptions`. If executed in an environment strictly enforcing Standalone Mode testing (SQLite memory DB), this test logic fails to compile or run.

## Recommendation
**DEPRECATE `sub_agent_queue`**

1. Ensure all `QueueManager` payloads and scheduling logic are migrated to use the `TaskQueue` interface backed by `ohc_job_queue`.
2. Remove the `QueueManager` struct and `sub_agent_queue` references from `src/server/queue.rs` to prevent accidental usage.
3. Remove the hardcoded `CREATE TABLE IF NOT EXISTS sub_agent_queue` calls from `db.rs` and `queue.rs`.
4. Update associated tests (`test_sub_agent_queue_isolation`) to utilize the new `ohc_job_queue` architecture or deprecate them entirely in favor of `ohc_job_queue_test.rs`.
