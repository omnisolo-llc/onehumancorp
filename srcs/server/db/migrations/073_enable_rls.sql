-- +goose Up
ALTER TABLE mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE mcp_servers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE ohc_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ohc_tasks ON ohc_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_ohc_tasks ON ohc_tasks;
ALTER TABLE ohc_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
ALTER TABLE mcp_servers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
ALTER TABLE mcp_audit_sync_log DISABLE ROW LEVEL SECURITY;
