CREATE TABLE IF NOT EXISTS settings (tenant_id TEXT PRIMARY KEY, sms_critical_phone TEXT, voice_receptionist_number TEXT, twilio_whatsapp_config JSONB, meta_whatsapp_config JSONB);
ALTER TABLE settings ADD COLUMN IF NOT EXISTS twilio_whatsapp_config JSONB;
ALTER TABLE settings ADD COLUMN IF NOT EXISTS meta_whatsapp_config JSONB;
