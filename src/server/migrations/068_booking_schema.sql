-- Migration 068: Booking Schema with tsrange and GiST

CREATE EXTENSION IF NOT EXISTS btree_gist;

CREATE TABLE IF NOT EXISTS bookable_resources (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    capacity INT DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS availability_slots (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    resource_id TEXT REFERENCES bookable_resources(id) ON DELETE CASCADE,
    slot_range tsrange NOT NULL,
    is_available BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- We need a robust GiST constraint to ensure no overlapping availability slots for the same resource
ALTER TABLE availability_slots
ADD CONSTRAINT prevent_overlapping_availability
EXCLUDE USING gist (
    resource_id WITH =,
    slot_range WITH &&
);

-- Create missing GiST constraints on bookings as well
ALTER TABLE bookings
ADD COLUMN IF NOT EXISTS resource_id TEXT REFERENCES bookable_resources(id) ON DELETE CASCADE,
ADD COLUMN IF NOT EXISTS time_range tsrange;

-- Populate time_range from start_time and end_time
UPDATE bookings SET time_range = tsrange(start_time, end_time) WHERE time_range IS NULL;

ALTER TABLE bookings
ADD CONSTRAINT prevent_double_bookings
EXCLUDE USING gist (
    resource_id WITH =,
    time_range WITH &&
);

ALTER TABLE bookable_resources ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_bookable_resources ON bookable_resources USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE availability_slots ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_availability_slots ON availability_slots USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
