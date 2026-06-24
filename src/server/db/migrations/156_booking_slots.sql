-- +goose Up
CREATE TABLE IF NOT EXISTS booking_slots (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_id TEXT,
    resource_id TEXT,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL DEFAULT 'available' CHECK (status IN ('available', 'soft_locked', 'booked')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_booking_slots_tenant_id ON booking_slots(tenant_id);
CREATE INDEX IF NOT EXISTS idx_booking_slots_service_id ON booking_slots(service_id);

ALTER TABLE booking_slots ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_slots ON booking_slots;
CREATE POLICY tenant_isolation_booking_slots
ON booking_slots
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE quotes ADD COLUMN IF NOT EXISTS service_id TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS proposed_slot_id TEXT;

-- +goose Down
ALTER TABLE quotes DROP COLUMN IF EXISTS proposed_slot_id;
ALTER TABLE quotes DROP COLUMN IF EXISTS service_id;

DROP POLICY IF EXISTS tenant_isolation_booking_slots ON booking_slots;
DROP TABLE IF EXISTS booking_slots CASCADE;
