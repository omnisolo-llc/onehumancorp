CREATE TABLE IF NOT EXISTS availability_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    business_hours JSONB NOT NULL DEFAULT '{}',
    exceptions JSONB NOT NULL DEFAULT '[]',
    timezone TEXT NOT NULL DEFAULT 'UTC',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS calendar_integrations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TIMESTAMPTZ,
    sync_metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- RLS
ALTER TABLE availability_schedules ENABLE ROW LEVEL SECURITY;
CREATE POLICY availability_schedules_tenant_isolation ON availability_schedules
    USING (tenant_id = current_setting('app.current_tenant_id', true));

ALTER TABLE calendar_integrations ENABLE ROW LEVEL SECURITY;
CREATE POLICY calendar_integrations_tenant_isolation ON calendar_integrations
    USING (tenant_id = current_setting('app.current_tenant_id', true));

-- Add payment_intent_id to bookings table if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name='bookings' AND column_name='payment_intent_id') THEN
        ALTER TABLE bookings ADD COLUMN payment_intent_id TEXT;
    END IF;
END $$;

-- Add deposit_required to services table if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name='services' AND column_name='deposit_required') THEN
        ALTER TABLE services ADD COLUMN deposit_required DECIMAL DEFAULT 0;
    END IF;
END $$;
