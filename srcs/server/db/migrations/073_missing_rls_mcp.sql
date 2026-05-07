-- +goose Up
ALTER TABLE mcp_servers ENABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers
    USING (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_ohc_tasks ON ohc_tasks;
CREATE POLICY tenant_isolation_ohc_tasks ON ohc_tasks
    USING (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
CREATE POLICY tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log
    USING (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
DROP POLICY IF EXISTS tenant_isolation_ohc_tasks ON ohc_tasks;
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;

ALTER TABLE mcp_audit_sync_log DISABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_servers DISABLE ROW LEVEL SECURITY;
