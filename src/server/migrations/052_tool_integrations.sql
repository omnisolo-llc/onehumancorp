CREATE TABLE IF NOT EXISTS tool_integrations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    api_url TEXT,
    integration_code TEXT,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tool_integrations_tenant_id ON tool_integrations(tenant_id);

ALTER TABLE tool_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id::text = current_setting('app.current_tenant', true));
