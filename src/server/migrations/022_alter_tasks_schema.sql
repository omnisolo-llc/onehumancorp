-- 022_alter_tasks_schema.sql

ALTER TABLE tasks ADD COLUMN metadata JSONB;
ALTER TABLE tasks ADD COLUMN tenant_id TEXT NOT NULL DEFAULT 'system';

-- Update the status check constraint
ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_status_check;
ALTER TABLE tasks ADD CONSTRAINT tasks_status_check CHECK (status IN ('PENDING', 'IN_PROGRESS', 'BLOCKED', 'DONE'));

CREATE INDEX idx_tasks_org_id ON tasks(tenant_id);
