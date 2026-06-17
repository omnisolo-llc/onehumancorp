-- +goose Up
-- Add scheduled status to bookings
ALTER TABLE bookings DROP CONSTRAINT IF EXISTS bookings_status_check;
ALTER TABLE bookings ADD CONSTRAINT bookings_status_check CHECK (status IN ('pending', 'pending_payment', 'confirmed', 'completed', 'cancelled', 'scheduled'));

-- Add additional columns to bookings for agentic rescheduling and notes
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS rescheduled_from_id TEXT;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS notes TEXT;

-- Availability blocks for native agentic booking
CREATE TABLE IF NOT EXISTS availability_blocks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_id TEXT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    resource_id TEXT REFERENCES booking_resources(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_availability_blocks_tenant_id ON availability_blocks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_availability_blocks_service_id ON availability_blocks(service_id);
CREATE INDEX IF NOT EXISTS idx_availability_blocks_resource_id ON availability_blocks(resource_id);
CREATE INDEX IF NOT EXISTS idx_availability_blocks_time ON availability_blocks(start_time, end_time);

ALTER TABLE availability_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
CREATE POLICY tenant_isolation_availability_blocks
ON availability_blocks
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
DROP TABLE IF EXISTS availability_blocks CASCADE;

ALTER TABLE bookings DROP COLUMN IF EXISTS rescheduled_from_id;
ALTER TABLE bookings DROP COLUMN IF EXISTS notes;

ALTER TABLE bookings DROP CONSTRAINT IF EXISTS bookings_status_check;
ALTER TABLE bookings ADD CONSTRAINT bookings_status_check CHECK (status IN ('pending', 'pending_payment', 'confirmed', 'completed', 'cancelled'));
