-- +goose Up
-- Enable Row Level Security on missed multi-tenant tables
ALTER TABLE mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_servers ENABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_tasks ENABLE ROW LEVEL SECURITY;

-- mcp_audit_sync_log
DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
CREATE POLICY tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log
    USING (tenant_id = current_setting('app.current_tenant', true));

-- mcp_servers
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers
    USING (tenant_id = current_setting('app.current_tenant', true));

-- ohc_tasks
DROP POLICY IF EXISTS tenant_isolation_ohc_tasks ON ohc_tasks;
CREATE POLICY tenant_isolation_ohc_tasks ON ohc_tasks
    USING (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
ALTER TABLE ohc_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_servers DISABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_audit_sync_log DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_tasks ON ohc_tasks;
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
