CREATE TABLE IF NOT EXISTS service_resource_requirements (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_id TEXT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_service_resource_requirements_tenant_id ON service_resource_requirements(tenant_id);

ALTER TABLE service_resource_requirements ENABLE ROW LEVEL SECURITY;
ALTER TABLE service_resource_requirements FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
CREATE POLICY tenant_isolation_service_resource_requirements ON service_resource_requirements
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
GRANT ALL PRIVILEGES ON service_resource_requirements TO ohc_bypassrls;


CREATE TABLE IF NOT EXISTS booking_resource_reservations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    booking_id TEXT NOT NULL REFERENCES bookings(id) ON DELETE CASCADE,
    resource_id TEXT NOT NULL REFERENCES booking_resources(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_booking_resource_reservations_tenant_id ON booking_resource_reservations(tenant_id);

ALTER TABLE booking_resource_reservations ENABLE ROW LEVEL SECURITY;
ALTER TABLE booking_resource_reservations FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resource_reservations ON booking_resource_reservations;
CREATE POLICY tenant_isolation_booking_resource_reservations ON booking_resource_reservations
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
GRANT ALL PRIVILEGES ON booking_resource_reservations TO ohc_bypassrls;
