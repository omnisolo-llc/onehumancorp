-- +goose Up
CREATE TABLE IF NOT EXISTS service_routes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_profile_id TEXT NOT NULL REFERENCES staff_profiles(id) ON DELETE CASCADE,
    route_date DATE NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'completed')),
    start_location_lat DOUBLE PRECISION,
    start_location_lng DOUBLE PRECISION,
    end_location_lat DOUBLE PRECISION,
    end_location_lng DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_service_routes_tenant_id ON service_routes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_service_routes_staff_date ON service_routes(tenant_id, staff_profile_id, route_date);

ALTER TABLE service_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
CREATE POLICY tenant_isolation_service_routes
ON service_routes
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS job_locations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_route_id TEXT NOT NULL REFERENCES service_routes(id) ON DELETE CASCADE,
    appointment_id TEXT NOT NULL REFERENCES appointments(id) ON DELETE CASCADE,
    sequence_order INTEGER NOT NULL,
    estimated_travel_time_mins INTEGER,
    distance_to_next_km DOUBLE PRECISION,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'en_route', 'on_site', 'completed', 'skipped')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(service_route_id, sequence_order)
);

CREATE INDEX IF NOT EXISTS idx_job_locations_tenant_id ON job_locations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_job_locations_route ON job_locations(tenant_id, service_route_id, sequence_order);

ALTER TABLE job_locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
CREATE POLICY tenant_isolation_job_locations
ON job_locations
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
DROP TABLE IF EXISTS job_locations CASCADE;

DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
DROP TABLE IF EXISTS service_routes CASCADE;
