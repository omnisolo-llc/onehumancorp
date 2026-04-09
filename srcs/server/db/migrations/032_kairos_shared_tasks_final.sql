-- We need to handle SQLite's lack of support for IF NOT EXISTS in ALTER TABLE
-- and the fact that 013 silently failed to add organization_id if shared_tasks existed.

-- In SQLite, we might get an error if organization_id doesn't exist when we try to create an index.
-- This usually means we need to conditionally add it or just re-create the table.
-- Let's just drop the table and re-create it since this is just tasks/queues! Wait, NO DROP TABLE.
-- What if we use a safe query that doesn't care if it's executed?
-- The test fails on CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
-- because organization_id doesn't exist in shared_tasks in the test.
-- Let's just avoid referencing organization_id in sqlite for this migration if we can, or add it.

ALTER TABLE shared_tasks ADD COLUMN organization_id TEXT DEFAULT 'system';
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
