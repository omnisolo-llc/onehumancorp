CREATE TABLE IF NOT EXISTS voice_agent_configs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    phone_number TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    primary_language TEXT NOT NULL DEFAULT 'en',
    custom_instructions TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE voice_agent_configs ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_voice_agent_configs ON voice_agent_configs
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
