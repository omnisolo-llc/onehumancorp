-- +goose Up
-- Strictly enforce tenant isolation, removing the insecure 'system' bypass backdoors.

-- Update policies for tasks
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Update policies for task_dependencies
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies
    USING (organization_id::text = current_setting('app.current_tenant', true));

-- Update policies for shared_tasks_decomposition
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition
    USING (organization_id::text = current_setting('app.current_tenant', true));

-- Update policies for consolidated_memory
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory
    USING (organization_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies
    USING (organization_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition
    USING (organization_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory
    USING (organization_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
