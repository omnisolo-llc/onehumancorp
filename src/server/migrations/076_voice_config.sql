CREATE TABLE IF NOT EXISTS voice_config (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL UNIQUE REFERENCES tenants(id) ON DELETE CASCADE,
    greeting TEXT NOT NULL,
    transfer_number TEXT,
    voice_type TEXT DEFAULT 'friendly',
    multi_lingual_enabled BOOLEAN DEFAULT false,
    twilio_number TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE voice_config ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Tenants can manage their own voice config" ON voice_config
    FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));
