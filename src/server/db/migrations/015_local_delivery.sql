-- Create delivery_zones table
CREATE TABLE IF NOT EXISTS delivery_zones (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    -- Simple geometry representation (JSON string) for sqlite compatibility
    -- PostGIS is ideal but sqlite needs a simpler representation
    polygon TEXT NOT NULL,
    flat_fee DECIMAL NOT NULL,
    min_order_value INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_delivery_zones_tenant_id ON delivery_zones(tenant_id);

ALTER TABLE delivery_zones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_zones ON delivery_zones;
CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Create delivery_tasks table
CREATE TABLE IF NOT EXISTS delivery_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    driver_id TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    estimated_arrival TIMESTAMPTZ,
    delivery_location TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_delivery_tasks_tenant_id ON delivery_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_delivery_tasks_status ON delivery_tasks(status);

ALTER TABLE delivery_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_tasks ON delivery_tasks;
CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Create route_plans table
CREATE TABLE IF NOT EXISTS route_plans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    delivery_date DATE NOT NULL,
    waypoint_sequence TEXT NOT NULL, -- JSON string representing sequence of tasks
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_route_plans_tenant_id ON route_plans(tenant_id);

ALTER TABLE route_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_plans ON route_plans;
CREATE POLICY tenant_isolation_route_plans ON route_plans USING (tenant_id::text = current_setting('app.current_tenant', true));
