-- Migration 218: Create whatsapp_channels table for native integration
CREATE TABLE IF NOT EXISTS whatsapp_channels (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    phone_number TEXT NOT NULL,
    phone_number_id TEXT NOT NULL,
    business_account_id TEXT NOT NULL,
    api_token TEXT NOT NULL, -- encrypted API token in production
    calling_enabled BOOLEAN DEFAULT FALSE,
    webhook_verify_token TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT unique_phone_number_id UNIQUE(tenant_id, phone_number_id)
);

ALTER TABLE whatsapp_channels ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_whatsapp_channels ON whatsapp_channels;
CREATE POLICY tenant_isolation_whatsapp_channels ON whatsapp_channels
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
