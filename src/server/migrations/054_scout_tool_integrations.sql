CREATE TABLE IF NOT EXISTS tool_integrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    api_url TEXT,
    integration_code TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Apply RLS based on the standard tenant isolation protocol
ALTER TABLE tool_integrations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = '');
