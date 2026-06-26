-- +goose Up
CREATE TABLE IF NOT EXISTS service_routes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    status TEXT NOT NULL DEFAULT 'planned' CHECK (status IN ('planned', 'in_progress', 'completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_service_routes_tenant_id ON service_routes(tenant_id);

ALTER TABLE service_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
CREATE POLICY tenant_isolation_service_routes
ON service_routes
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS route_stops (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    route_id TEXT NOT NULL REFERENCES service_routes(id) ON DELETE CASCADE,
    appointment_id TEXT REFERENCES appointments(id) ON DELETE SET NULL,
    estimated_arrival_time TIMESTAMPTZ,
    estimated_departure_time TIMESTAMPTZ,
    sequence_order INT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'en_route', 'in_progress', 'completed', 'skipped')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_route_stops_tenant_id ON route_stops(tenant_id);

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
