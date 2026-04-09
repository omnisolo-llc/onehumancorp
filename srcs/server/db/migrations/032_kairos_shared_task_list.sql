-- Add organization_id if it does not exist (some test environments might not run all migrations in sequence or have different schemas).
-- Since SQLite ALTER TABLE is limited, we handle this conditionally.
-- Actually, the error says "no such column: organization_id", but we saw organization_id in 013.
-- Wait, 013 is `organization_id VARCHAR NOT NULL`. The error happens during test setup in `TestAutoDreamConsolidateEpoch` which uses `dbWrapper.Provider().ApplyMigrations()`. Let's just create the table in 032 if missing, but it is there.
-- Let's just make sure we check if organization_id exists, or better, we can recreate the table if needed. Or we just drop the index if exists and then create it on existing columns?
-- Oh, maybe the table in `013_shared_tasks.sql` was not run or maybe `shared_tasks` in the DB doesn't have `organization_id` because `shared_tasks` was created in `007_teammate_mesh_and_autodream.sql` which did *not* have `organization_id`!!!
-- Let's check 007!
-- In 007: `CREATE TABLE IF NOT EXISTS shared_tasks (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), mission_id TEXT NOT NULL, title TEXT NOT NULL, ...)` -> NO organization_id!
-- And `013_shared_tasks.sql` says `CREATE TABLE IF NOT EXISTS shared_tasks` -> so it's ignored because the table already exists from 007!
-- Same for `021_kairos_orchestration.sql` (`CREATE TABLE IF NOT EXISTS shared_tasks`)!
-- So `organization_id` was NEVER added to `shared_tasks`!
-- Let's add it via ALTER TABLE!

ALTER TABLE shared_tasks ADD COLUMN organization_id TEXT NOT NULL DEFAULT 'system';
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
