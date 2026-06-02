-- +goose Up
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS delivery_zones (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    zone_polygon GEOMETRY(POLYGON, 4326),
    flat_fee DECIMAL DEFAULT 0,
    min_order_value DECIMAL DEFAULT 0,
    max_daily_deliveries INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS delivery_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT,
    driver_id TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'IN_TRANSIT', 'DELIVERED', 'FAILED')),
    estimated_arrival TIMESTAMPTZ,
    delivery_location GEOMETRY(POINT, 4326),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS route_plans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    delivery_date DATE NOT NULL,
    waypoint_sequence JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE delivery_zones ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE delivery_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE route_plans ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_route_plans ON route_plans
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS route_plans CASCADE;
DROP TABLE IF EXISTS delivery_tasks CASCADE;
DROP TABLE IF EXISTS delivery_zones CASCADE;
