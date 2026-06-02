CREATE TABLE IF NOT EXISTS couriers (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    name TEXT NOT NULL,
    phone TEXT NOT NULL,
    vehicle_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'OFFLINE',
    location GEOMETRY(Point, 4326),
    stripe_account_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_couriers_org ON couriers(organization_id);
CREATE INDEX IF NOT EXISTS idx_couriers_location ON couriers USING GIST (location);

ALTER TABLE couriers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_couriers ON couriers;
CREATE POLICY tenant_isolation_couriers ON couriers USING (organization_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS delivery_jobs (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    courier_id UUID REFERENCES couriers(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'AVAILABLE',
    pickup_location GEOMETRY(Point, 4326) NOT NULL,
    delivery_location GEOMETRY(Point, 4326) NOT NULL,
    payout_cents BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_delivery_jobs_org ON delivery_jobs(organization_id);
CREATE INDEX IF NOT EXISTS idx_delivery_jobs_status ON delivery_jobs(status);
CREATE INDEX IF NOT EXISTS idx_delivery_jobs_courier ON delivery_jobs(courier_id);

ALTER TABLE delivery_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_jobs ON delivery_jobs;
CREATE POLICY tenant_isolation_delivery_jobs ON delivery_jobs USING (organization_id::text = current_setting('app.current_tenant', true));
