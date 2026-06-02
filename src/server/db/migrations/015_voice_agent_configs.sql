CREATE TABLE IF NOT EXISTS voice_agent_configs (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    phone_number TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    primary_language TEXT DEFAULT 'English',
    custom_instructions TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_voice_agent_configs_org ON voice_agent_configs(organization_id);

ALTER TABLE voice_agent_configs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_voice_agent_configs ON voice_agent_configs;
CREATE POLICY tenant_isolation_voice_agent_configs ON voice_agent_configs USING (organization_id::text = current_setting('app.current_tenant', true));
