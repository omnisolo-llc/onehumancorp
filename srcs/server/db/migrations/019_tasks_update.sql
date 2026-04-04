ALTER TABLE tasks RENAME COLUMN parent_task_id TO parent_id;
ALTER TABLE tasks RENAME COLUMN agent_id TO assigned_agent_id;
ALTER TABLE tasks ADD COLUMN title TEXT;
ALTER TABLE tasks ADD COLUMN description TEXT;
ALTER TABLE tasks ADD COLUMN metadata JSONB;
ALTER TABLE tasks ADD COLUMN organization_id TEXT;

ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_status_check;
ALTER TABLE tasks ADD CONSTRAINT tasks_status_check CHECK (status IN ('PENDING', 'IN_PROGRESS', 'BLOCKED', 'DONE', 'RUNNING', 'COMPLETED', 'FAILED'));

CREATE INDEX IF NOT EXISTS idx_tasks_org ON tasks(organization_id);
