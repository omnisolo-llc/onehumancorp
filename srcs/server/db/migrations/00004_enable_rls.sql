-- +goose Up
-- Enable Row Level Security on core multi-tenant tables
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_servers ENABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;

-- Ensure organization_id exists on all multi-tenant tables for uniform policy enforcement
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE state_machine_transitions ADD COLUMN IF NOT EXISTS organization_id VARCHAR;

-- Create Tenant Isolation Policies
-- shared_tasks
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks
    USING (organization_id = current_setting('app.current_tenant', true));

-- tenants (A tenant can only see its own record)
DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
CREATE POLICY tenant_isolation_tenants ON tenants
    USING (id::text = current_setting('app.current_tenant', true));

-- autodream_memories
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories
    USING (organization_id = current_setting('app.current_tenant', true));

-- swarm_tasks
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks
    USING (organization_id = current_setting('app.current_tenant', true));

-- state_machine_transitions
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions
    USING (organization_id = current_setting('app.current_tenant', true));

-- mcp_servers
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers
    USING (tenant_id = current_setting('app.current_tenant', true));

-- ohc_tasks
DROP POLICY IF EXISTS tenant_isolation_ohc_tasks ON ohc_tasks;
CREATE POLICY tenant_isolation_ohc_tasks ON ohc_tasks
    USING (organization_id = current_setting('app.current_tenant', true));

-- mcp_audit_sync_log
DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
CREATE POLICY tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log
    USING (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
ALTER TABLE mcp_audit_sync_log DISABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_servers DISABLE ROW LEVEL SECURITY;
ALTER TABLE state_machine_transitions DISABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_memories DISABLE ROW LEVEL SECURITY;
ALTER TABLE tenants DISABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
DROP POLICY IF EXISTS tenant_isolation_ohc_tasks ON ohc_tasks;
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
