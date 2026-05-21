CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    agent_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_config_sync_log FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log USING (tenant_id::text = current_setting('app.current_tenant', true));
