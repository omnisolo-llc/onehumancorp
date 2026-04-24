1. **Create the SQL migration file for `shared_tasks`:**
   - Create `src/server/db/migrations/063_shared_tasks.sql` based on the provided Design Doc schema.
   - The schema is:
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
```

2. **Add migration to `embedsrcs`:**
   - Add `"migrations/063_shared_tasks.sql"` to `embedsrcs` in `src/server/db/BUILD.bazel`.

3. **Refactor `tasks_db.go`:**
   - Modify `TasksDB.ClaimTask` at line 375 to correctly use `telemetry.RecordPostgresLockContention(ctx, "claim_task")` when `TryLock` fails.
     ```go
     if to.dbProvider.IsSQLite() {
         if !to.mu.TryLock() {
             telemetry.RecordPostgresLockContention(ctx, "claim_task")
             to.mu.Lock()
         }
         defer to.mu.Unlock()
         // ...
     ```

4. **Testing:**
   - Run `bazelisk test //src/server/orchestration/...` and `bazelisk test //src/server/db/...`.
   - Update `tasks_db_test.go` or other tests if required.

5. **Pre-commit step:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
