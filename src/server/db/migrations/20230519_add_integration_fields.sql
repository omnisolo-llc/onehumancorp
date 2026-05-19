ALTER TABLE tenants ADD COLUMN IF NOT EXISTS meta_graph_token TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS google_calendar_token TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS google_calendar_refresh TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS zoom_token TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS sms_reminders_enabled BOOLEAN DEFAULT FALSE;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS sms_phone TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS sendgrid_api_key TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS shippo_api_key TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS mercadopago_token TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS meta_graph_page_id TEXT;

CREATE TABLE IF NOT EXISTS unified_inbox_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    external_sender_id TEXT NOT NULL,
    text TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
