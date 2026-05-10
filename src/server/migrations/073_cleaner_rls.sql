-- 073_cleaner_rls.sql
-- Missing RLS enforcement audit

ALTER TABLE IF EXISTS action_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_approvals ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bus_checkpoints ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bus_locks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bus_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS local_execution_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS local_queue_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mcp_servers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mesh_checkpoints ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mesh_locks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mesh_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mesh_presence ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS revoked_tokens ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS roles ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_action_queue ON action_queue;
CREATE POLICY tenant_isolation_action_queue ON action_queue
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_agent_approvals ON agent_approvals;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_bus_checkpoints ON bus_checkpoints;
CREATE POLICY tenant_isolation_bus_checkpoints ON bus_checkpoints
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_bus_locks ON bus_locks;
CREATE POLICY tenant_isolation_bus_locks ON bus_locks
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_bus_messages ON bus_messages;
CREATE POLICY tenant_isolation_bus_messages ON bus_messages
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_local_execution_results ON local_execution_results;
CREATE POLICY tenant_isolation_local_execution_results ON local_execution_results
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_local_queue_jobs ON local_queue_jobs;
CREATE POLICY tenant_isolation_local_queue_jobs ON local_queue_jobs
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_mesh_checkpoints ON mesh_checkpoints;
CREATE POLICY tenant_isolation_mesh_checkpoints ON mesh_checkpoints
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_mesh_locks ON mesh_locks;
CREATE POLICY tenant_isolation_mesh_locks ON mesh_locks
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_mesh_messages ON mesh_messages;
CREATE POLICY tenant_isolation_mesh_messages ON mesh_messages
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_mesh_presence ON mesh_presence;
CREATE POLICY tenant_isolation_mesh_presence ON mesh_presence
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_revoked_tokens ON revoked_tokens;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_roles ON roles;
CREATE POLICY tenant_isolation_roles ON roles
    USING (tenant_id::text = current_setting('app.current_tenant', true));
