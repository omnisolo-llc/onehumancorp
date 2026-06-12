CREATE EXTENSION IF NOT EXISTS postgis;
CREATE TABLE IF NOT EXISTS delivery_zones (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    polygon GEOMETRY(Polygon, 4326),
    flat_fee_cents BIGINT NOT NULL DEFAULT 0,
    min_order_value_cents BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_delivery_zones_org ON delivery_zones(organization_id);
CREATE INDEX IF NOT EXISTS idx_delivery_zones_polygon ON delivery_zones USING GIST (polygon);
ALTER TABLE delivery_zones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_zones ON delivery_zones;
CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS route_plans (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    delivery_date DATE NOT NULL,
    waypoint_sequence JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_route_plans_org_date ON route_plans(organization_id, delivery_date);
ALTER TABLE route_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_plans ON route_plans;
CREATE POLICY tenant_isolation_route_plans ON route_plans USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS delivery_tasks (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    driver_id TEXT,
    route_plan_id UUID REFERENCES route_plans(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    estimated_arrival TIMESTAMPTZ,
    delivery_location GEOMETRY(Point, 4326),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_delivery_tasks_org ON delivery_tasks(organization_id);
CREATE INDEX IF NOT EXISTS idx_delivery_tasks_order ON delivery_tasks(order_id);
CREATE INDEX IF NOT EXISTS idx_delivery_tasks_route_plan ON delivery_tasks(route_plan_id);
CREATE INDEX IF NOT EXISTS idx_delivery_tasks_location ON delivery_tasks USING GIST (delivery_location);
ALTER TABLE delivery_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_tasks ON delivery_tasks;
CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));