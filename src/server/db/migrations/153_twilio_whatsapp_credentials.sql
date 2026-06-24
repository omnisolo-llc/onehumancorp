-- Add twilio credentials and phone number mapping for whatsapp integration
ALTER TABLE settings ADD COLUMN IF NOT EXISTS twilio_whatsapp_account_sid TEXT;
ALTER TABLE settings ADD COLUMN IF NOT EXISTS twilio_whatsapp_auth_token TEXT;
ALTER TABLE settings ADD COLUMN IF NOT EXISTS twilio_whatsapp_phone_number TEXT;
