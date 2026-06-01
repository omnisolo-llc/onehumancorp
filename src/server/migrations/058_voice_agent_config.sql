-- Migration 058: Voice Agent Config

CREATE TABLE IF NOT EXISTS voice_agent_configs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    phone_number TEXT,
    is_enabled BOOLEAN DEFAULT false,
    allow_booking BOOLEAN DEFAULT false,
    allow_sms_links BOOLEAN DEFAULT false,
    primary_language TEXT DEFAULT 'English',
    custom_instructions TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE voice_agent_configs ENABLE ROW LEVEL SECURITY;

-- Create policy for tenant isolation
CREATE POLICY tenant_isolation_voice_agent_configs ON voice_agent_configs
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Add trigger for updated_at
CREATE TRIGGER set_timestamp_voice_agent_configs
    BEFORE UPDATE ON voice_agent_configs
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();