CREATE TABLE IF NOT EXISTS settings (
    tenant_id TEXT PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    sms_critical_phone TEXT,
    voice_receptionist_number TEXT,
    whatsapp_business_number TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_settings_whatsapp ON settings(whatsapp_business_number);

ALTER TABLE settings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_settings ON settings;
CREATE POLICY tenant_isolation_settings
ON settings
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
