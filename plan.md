1.  **Fix Migration**: Remove `DROP TABLE IF EXISTS shared_tasks;` from the migration `041_shared_tasks_schema.sql` and ensure it adds the JSONB column correctly or modifies the table safely instead. Wait, `20260410_kairos_tasks.sql` already creates `shared_tasks` with `dependencies TEXT`. `013_shared_tasks.sql` already creates `shared_tasks`. The review says "dropping the table... will wipe out any existing production data. Migrations should perform safe, additive schema changes."
    Let's check the schema again. `013_shared_tasks.sql` has `dependencies`? No.
    `013_shared_tasks.sql`: `payload JSONB`
    `20260410_kairos_tasks.sql`: `dependencies TEXT`, `parent_plan_id TEXT`, `agent_id TEXT`
    If `shared_tasks` exists, I should just ALTER it to add `dependencies JSONB` or use the existing `dependencies TEXT` (which maybe is a JSON string). Actually, `20260410_kairos_tasks.sql` uses `dependencies TEXT`. I'll create `041_shared_tasks_schema.sql` as an ALTER TABLE if needed, or just remove the `DROP TABLE`. Wait, what is the exact schema in SQLite vs Postgres? The review says: "Migrations should perform safe, additive schema changes."
    I will rewrite `041_shared_tasks_schema.sql` to add the columns if they don't exist:
    ```sql
    ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;
    ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB;
    ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent_id TEXT;
    CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
    ```

2.  **Fix DAG Logic in `ClaimTask`**:
    The review says: "The patched query ... entirely ignoring the new `dependencies` JSONB column. This breaks the sequential task execution orchestration... The query logic must be rewritten to respect the `dependencies` JSONB field before claiming a task."
    How to check JSONB array in Postgres?
    `AND NOT EXISTS (SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep_id JOIN shared_tasks d ON d.id = dep_id WHERE d.status != 'COMPLETED')`
    For SQLite: SQLite has `json_each`.
    `AND NOT EXISTS (SELECT 1 FROM json_each(st.dependencies) AS dep JOIN shared_tasks d ON d.id = dep.value WHERE d.status != 'COMPLETED')`
    Since we have two different SQL dialects (Postgres vs SQLite), we can use the `dbProvider.IsSQLite()` split.

3.  **Update `tasks_db_test.go`**:
    Add back the dependency tests. Add tasks with `dependencies` set to `["task-1"]` and verify that `ClaimTask` only claims them when `task-1` is `COMPLETED`.

4.  **Remove `plan.md`**:
    `rm plan.md`

5.  **Mark Mission DONE**:
    Update the frontmatter of `.agent-task/missions/2026-04-10T09-01-46Z_kairos_shared_task_list_schema.md` from `IN_PROGRESS` to `DONE`.

6.  Run Tests, Pre-commit, Submit.
