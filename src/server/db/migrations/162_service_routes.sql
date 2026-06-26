CREATE TABLE IF NOT EXISTS service_routes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    route_date DATE NOT NULL,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS route_stops (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    route_id TEXT REFERENCES service_routes(id) ON DELETE CASCADE,
    appointment_id TEXT REFERENCES appointments(id) ON DELETE CASCADE,
    sequence INTEGER NOT NULL,
    estimated_arrival TIMESTAMPTZ,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE service_routes ENABLE ROW LEVEL SECURITY;
CREATE POLICY service_routes_tenant_isolation ON service_routes
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE route_stops ENABLE ROW LEVEL SECURITY;
CREATE POLICY route_stops_tenant_isolation ON route_stops
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
