CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
    id SERIAL PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    config_key TEXT NOT NULL,
    config_value TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, config_key)
);
CREATE INDEX IF NOT EXISTS idx_mcp_config_sync_log_tenant_id ON mcp_config_sync_log(tenant_id);
ALTER TABLE mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log;
CREATE POLICY tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));