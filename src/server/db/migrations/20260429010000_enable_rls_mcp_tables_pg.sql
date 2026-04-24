-- Empty migration since RLS unsupported in sqlite, but in PG we run this:
ALTER TABLE mcp_servers ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON mcp_servers USING (tenant_id = current_setting('app.current_tenant')::uuid);

ALTER TABLE mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON mcp_config_sync_log USING (tenant_id = current_setting('app.current_tenant')::varchar);

ALTER TABLE mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON mcp_audit_sync_log USING (tenant_id = current_setting('app.current_tenant')::varchar);

ALTER TABLE local_mcp_rag_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON local_mcp_rag_tasks USING (tenant_id = current_setting('app.current_tenant')::text);
