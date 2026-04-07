1. **Create the SQL migration file**
    * Create `srcs/server/db/migrations/030_kairos_shared_tasks.sql` (Actually already done!).
2. **Update BUILD.bazel**
    * Add `migrations/030_kairos_shared_tasks.sql` to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. **Examine `srcs/server/orchestration/tasks.go` and `tasks_db.go`**
    * Determine if `tasks.go` or `tasks_db.go` is where I should add the new features. It looks like `tasks.go` handles some similar things. The mission says to create the data access layer in `srcs/server/orchestration/tasks_db.go` and implement a `ClaimTask` method.
4. **Implement `ClaimTask` method**
    * In `srcs/server/orchestration/tasks_db.go` (create it if it doesn't exist).
    * It must handle claiming tasks and prevent concurrent assignment conflicts using `SELECT * FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED` for PostgreSQL.
    * For SQLite Standalone mode, use application-level mutexes (or simple transaction isolation) to claim the task safely.
5. **Create Unit Tests**
    * Create `srcs/server/orchestration/tasks_db_test.go` with unit tests for `tasks_db.go`.
    * Use `context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)` if simulating authentication claims.
6. **Pre-commit step**
    * Complete pre-commit steps to make sure proper testing, verifications, reviews and reflections are done.
7. **Submit the change**
    * Run `bazelisk test //srcs/server/orchestration/...` and wait for everything to pass.
    * Submit.
