-- +goose Up
-- Enable RLS on core multi-tenant tables
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;

-- Tenants Policy: Can only see their own row
CREATE POLICY tenant_isolation_policy ON tenants
    USING (id::text = current_setting('app.current_tenant', true));

-- Shared Tasks Policy: Can only see tasks belonging to their organization
CREATE POLICY task_isolation_policy ON shared_tasks
    USING (organization_id = current_setting('app.current_tenant', true));

-- AutoDream Memories Policy: Can only see memories belonging to their organization
CREATE POLICY memory_isolation_policy ON autodream_memories
    USING (organization_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS memory_isolation_policy ON autodream_memories;
DROP POLICY IF EXISTS task_isolation_policy ON shared_tasks;
DROP POLICY IF EXISTS tenant_isolation_policy ON tenants;

ALTER TABLE autodream_memories DISABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE tenants DISABLE ROW LEVEL SECURITY;
