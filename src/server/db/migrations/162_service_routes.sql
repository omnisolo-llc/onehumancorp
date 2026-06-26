-- +goose Up
CREATE TABLE IF NOT EXISTS service_routes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_profile_id TEXT REFERENCES staff_profiles(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'completed')),
    start_location_lat DOUBLE PRECISION,
    start_location_lng DOUBLE PRECISION,
    end_location_lat DOUBLE PRECISION,
    end_location_lng DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_service_routes_tenant_id ON service_routes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_service_routes_staff_date ON service_routes(tenant_id, staff_profile_id, date);

ALTER TABLE service_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
CREATE POLICY tenant_isolation_service_routes
ON service_routes
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS route_stops (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_route_id TEXT NOT NULL REFERENCES service_routes(id) ON DELETE CASCADE,
    appointment_id TEXT REFERENCES appointments(id) ON DELETE CASCADE,
    stop_order INTEGER NOT NULL,
    estimated_arrival_time TIMESTAMPTZ,
    estimated_departure_time TIMESTAMPTZ,
    travel_time_mins INTEGER,
    travel_distance_meters INTEGER,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'arrived', 'completed', 'skipped')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_route_stops_tenant_id ON route_stops(tenant_id);
CREATE INDEX IF NOT EXISTS idx_route_stops_route_order ON route_stops(tenant_id, service_route_id, stop_order);

ALTER TABLE route_stops ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_stops ON route_stops;
CREATE POLICY tenant_isolation_route_stops
ON route_stops
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_route_stops ON route_stops;
DROP TABLE IF EXISTS route_stops CASCADE;

DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
DROP TABLE IF EXISTS service_routes CASCADE;
