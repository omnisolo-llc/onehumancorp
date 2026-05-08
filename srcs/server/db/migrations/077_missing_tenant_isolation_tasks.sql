-- +goose Up
-- Add tenant_id to tasks if it doesn't exist
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;

-- Add policies for tasks
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Add policies for task_dependencies
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies
    USING (organization_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Add policies for shared_tasks_decomposition
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition
    USING (organization_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Add policies for consolidated_memory
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory
    USING (organization_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;

ALTER TABLE consolidated_memory DISABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_decomposition DISABLE ROW LEVEL SECURITY;
ALTER TABLE task_dependencies DISABLE ROW LEVEL SECURITY;
ALTER TABLE tasks DISABLE ROW LEVEL SECURITY;

ALTER TABLE tasks DROP COLUMN IF EXISTS tenant_id;
