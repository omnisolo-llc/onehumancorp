1. **Database Schema Setup**
   - Create a new migration file (e.g., `srcs/server/db/migrations/20260429000000_kairos_tasks_dag.sql` and `20260429000000_kairos_tasks_dag_sqlite.sql`)
   - Ensure the `task_dependencies` join table exists with fields `task_id` (UUID/TEXT) and `depends_on_task_id` (UUID/TEXT) with a primary key constraint on both.
   - Update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.

2. **Update Task Claiming Logic (`srcs/server/orchestration/tasks.go`)**
   - Modify `ClaimTask(ctx context.Context, taskID, agentID string) (*SharedTask, error)` to evaluate dependencies via a JOIN query on the new `task_dependencies` table, replacing the JSON array `st.dependencies` logic.
   - For PostgreSQL: Open a transaction with `tx, err := tm.db.Begin(ctx)` and use `FOR UPDATE SKIP LOCKED`.
   - For SQLite: Since SQLite doesn't support `FOR UPDATE SKIP LOCKED`, check if `pool.IsSQLite()` is true. If it is, use a two-step approach:
     - `SELECT id FROM shared_tasks ... LIMIT 1`
     - `UPDATE shared_tasks SET status = 'IN_PROGRESS' ... WHERE id = <fetched_id>`
   - To prevent lock errors, ensure proper transactions. As explicitly requested, avoid calling `.RowsAffected()`, and instead return `(int64, error)` from the query operation via returning the id directly.

3. **Check and Apply Multitenant Restrictions & Lock Contention**
   - Ensure `organization_id` is strictly enforced in all task queries.
   - Handle lock contention errors by explicitly checking for both `'database is locked'` and `'SQLITE_BUSY'`.

4. **Verify / Write Tests**
   - Check `srcs/server/orchestration/tasks_test.go` to confirm unit tests pass.
   - Add/verify tests achieving >90% coverage for SQLite and PostgreSQL behavior utilizing `sqlite://file::memory:?cache=shared` for the test DB.
   - For DB operations, verify that `(int64, error)` pattern is used if returning row counts, without calling `.RowsAffected()`.

5. **Complete pre-commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit Code**
