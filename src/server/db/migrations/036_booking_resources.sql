-- Unified Booking Resources

CREATE TABLE IF NOT EXISTS resources (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT NOT NULL, -- "staff", "equipment", "room", etc.
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_resources_tenant_id ON resources(tenant_id);

ALTER TABLE resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_resources ON resources;
CREATE POLICY tenant_isolation_resources
ON resources
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS service_resources (
    service_id TEXT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    resource_id TEXT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (service_id, resource_id)
);

CREATE INDEX IF NOT EXISTS idx_service_resources_tenant_id ON service_resources(tenant_id);

ALTER TABLE service_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_resources ON service_resources;
CREATE POLICY tenant_isolation_service_resources
ON service_resources
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS booking_resources (
    booking_id TEXT NOT NULL REFERENCES bookings(id) ON DELETE CASCADE,
    resource_id TEXT NOT NULL REFERENCES resources(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    PRIMARY KEY (booking_id, resource_id)
);

CREATE INDEX IF NOT EXISTS idx_booking_resources_tenant_id ON booking_resources(tenant_id);

ALTER TABLE booking_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
CREATE POLICY tenant_isolation_booking_resources
ON booking_resources
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Add resource_id to availability_blocks
ALTER TABLE availability_blocks ADD COLUMN IF NOT EXISTS resource_id TEXT REFERENCES resources(id) ON DELETE CASCADE;

-- Add service_id to bookings
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS service_id TEXT REFERENCES services(id) ON DELETE CASCADE;
