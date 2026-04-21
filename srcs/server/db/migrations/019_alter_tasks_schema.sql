ALTER TABLE tasks ADD COLUMN metadata JSONB;
ALTER TABLE tasks ADD COLUMN organization_id TEXT NOT NULL DEFAULT 'system';

-- Update the status check constraint
-- SQLite does not support DROP CONSTRAINT, but RunMigrations strips it out for SQLite.
ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_status_check;
ALTER TABLE tasks ADD CONSTRAINT tasks_status_check CHECK (status IN ('PENDING', 'IN_PROGRESS', 'BLOCKED', 'DONE'));

CREATE INDEX IF NOT EXISTS idx_tasks_org_id ON tasks(organization_id);
