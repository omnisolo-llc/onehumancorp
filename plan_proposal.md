1. **Database Schema Setup**
   - Use `run_in_bash_session` to write new migration files (`srcs/server/db/migrations/20260429000000_kairos_tasks_dag_pg.sql` and `20260429000000_kairos_tasks_dag_sqlite.sql`) ensuring the `task_dependencies` join table exists.
   - Use `run_in_bash_session` to `sed` `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.
   - Use `read_file` to verify the creation and modification of these files.

2. **Update Task Claiming Logic (`srcs/server/orchestration/tasks.go` and `srcs/server/orchestration/tasks_db.go`)**
   - Use `replace_with_git_merge_diff` on `srcs/server/orchestration/tasks.go` and `srcs/server/orchestration/tasks_db.go` to modify the `ClaimTask` logic.
   - The logic will evaluate dependencies via a JOIN query on the new `task_dependencies` table, replacing the JSON array `st.dependencies` logic.
   - For PostgreSQL: Open a transaction with `tx, err := tm.db.Begin(ctx)` and use `FOR UPDATE SKIP LOCKED`.
   - For SQLite: Since SQLite doesn't support `FOR UPDATE SKIP LOCKED`, check if `pool.IsSQLite()` is true. If it is, use a two-step approach:
     `SELECT id FROM shared_tasks ... LIMIT 1` then `UPDATE shared_tasks SET status = 'IN_PROGRESS' ... WHERE id = <fetched_id>`
   - To prevent lock errors, ensure proper transactions. As explicitly requested, avoid calling `.RowsAffected()`, and instead return `(int64, error)` from the query operation via returning the id directly. Ensure `organization_id` is strictly enforced.
   - Use `read_file` to verify the code modifications.

3. **Verify / Write Tests**
   - Use `replace_with_git_merge_diff` to add unit tests in `srcs/server/orchestration/tasks_db_test.go` and `srcs/server/orchestration/tasks_test.go` verifying SQLite and PostgreSQL behaviors, and the `(int64, error)` pattern.
   - Use `read_file` to verify the code modifications.

4. **Run tests**
   - Use `run_in_bash_session` to execute `bazelisk test //...` to verify all tests pass.

5. **Complete pre-commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit Code**
   - Submit via `submit`.
