1. **Parity Audit:** Investigate lock contention/missing locks.
   - Added `locked_until` column to `shared_tasks` via `20260428000001_shared_tasks_locked_until.sql` migration to fix lock contention issues in SQLite and Postgres environments. Verified it fixed DB tests.
   - Replaced `autodream_memories` with `consolidated_memory` in Go DB implementations to fix migrations logic since `autodream_memories` is deprecated and replaced.
   - Restored `task_dependencies` join table via fixing migrations: `20260424030000_shared_tasks_dependencies_pg.sql` and `20260424030000_shared_tasks_dependencies_sqlite.sql`.
2. **Chaos Engineering Tests Parity Audit**:
   - Fixed tests in `src/server/orchestration/queue/queue_manager_loop_test.go` missing migration runner logic that broke in standalone mode by modifying it to run migrations beforehand.
3. **Run all tests to verify**
   - Executed `bazelisk test //src/server/...` and verified all tests passed successfully!
