CREATE TABLE IF NOT EXISTS delivery_routes (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    driver_id TEXT,
    status TEXT NOT NULL DEFAULT 'planning',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_delivery_routes_organization_id ON delivery_routes(organization_id);

ALTER TABLE delivery_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_routes ON delivery_routes;
CREATE POLICY tenant_isolation_delivery_routes ON delivery_routes USING (organization_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS route_stops (
    id TEXT PRIMARY KEY,
    route_id TEXT NOT NULL REFERENCES delivery_routes(id) ON DELETE CASCADE,
    organization_id TEXT NOT NULL,
    order_id TEXT,
    address TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    eta_ms BIGINT,
    sort_order INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_route_stops_route_id ON route_stops(route_id);
CREATE INDEX IF NOT EXISTS idx_route_stops_organization_id ON route_stops(organization_id);

ALTER TABLE route_stops ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_stops ON route_stops;
CREATE POLICY tenant_isolation_route_stops ON route_stops USING (organization_id::text = current_setting('app.current_tenant', true));
