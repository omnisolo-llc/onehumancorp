CREATE TABLE IF NOT EXISTS availability_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    day_of_week INTEGER NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_availability_schedules_tenant ON availability_schedules(tenant_id);

ALTER TABLE availability_schedules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_schedules ON availability_schedules;
CREATE POLICY tenant_isolation_availability_schedules
ON availability_schedules
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Update bookings
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS deposit_amount_cents BIGINT;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS deposit_payment_intent_id TEXT;
