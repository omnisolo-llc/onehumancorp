-- +goose Up
-- Enable Row Level Security on missing multi-tenant tables
ALTER TABLE IF EXISTS mcp_servers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
CREATE POLICY tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
ALTER TABLE IF EXISTS mcp_servers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
ALTER TABLE IF EXISTS mcp_audit_sync_log DISABLE ROW LEVEL SECURITY;
