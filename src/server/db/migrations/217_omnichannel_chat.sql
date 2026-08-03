CREATE TABLE IF NOT EXISTS channel_twilio_sms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    account_sid TEXT NOT NULL,
    auth_token TEXT NOT NULL,
    messaging_service_sid TEXT,
    phone_number TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

ALTER TABLE channel_twilio_sms ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_channel_twilio_sms
    ON channel_twilio_sms
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE TABLE IF NOT EXISTS channel_whatsapp (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    business_management_token TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    message_templates JSONB,
    phone_number_health JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

ALTER TABLE channel_whatsapp ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_channel_whatsapp
    ON channel_whatsapp
    USING (tenant_id = current_setting('app.current_tenant_id', true));
