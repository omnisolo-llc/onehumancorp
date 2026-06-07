-- +goose Up
-- Add deposit_required to services
ALTER TABLE services ADD COLUMN IF NOT EXISTS deposit_required BIGINT DEFAULT 0;

-- Add availability_rules
CREATE TABLE IF NOT EXISTS availability_rules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    day_of_week INTEGER,
    start_time TEXT,
    end_time TEXT,
    is_available BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE availability_rules ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_availability_rules ON availability_rules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Add calendar_sync_connections
CREATE TABLE IF NOT EXISTS calendar_sync_connections (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE calendar_sync_connections ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_calendar_sync_connections ON calendar_sync_connections USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Add payment_intent_id to bookings
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS payment_intent_id TEXT;
