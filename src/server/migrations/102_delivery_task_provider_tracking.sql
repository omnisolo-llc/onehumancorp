CREATE TABLE IF NOT EXISTS delivery_tasks (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    driver_id TEXT,
    route_plan_id UUID,
    status TEXT NOT NULL DEFAULT 'PENDING',
    estimated_arrival TIMESTAMPTZ,
    delivery_location_lat DOUBLE PRECISION,
    delivery_location_lng DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_delivery_tasks_org ON delivery_tasks(organization_id);
CREATE INDEX IF NOT EXISTS idx_delivery_tasks_order ON delivery_tasks(order_id);
CREATE INDEX IF NOT EXISTS idx_delivery_tasks_location ON delivery_tasks(organization_id, delivery_location_lat, delivery_location_lng);

ALTER TABLE delivery_tasks ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_delivery_tasks ON delivery_tasks;
CREATE POLICY tenant_isolation_delivery_tasks
ON delivery_tasks
USING (organization_id::text = current_setting('app.current_tenant', true))
WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE delivery_tasks
ADD COLUMN IF NOT EXISTS provider TEXT;

ALTER TABLE delivery_tasks
ADD COLUMN IF NOT EXISTS provider_delivery_id TEXT;

ALTER TABLE delivery_tasks
ADD COLUMN IF NOT EXISTS delivery_location_lat DOUBLE PRECISION;

ALTER TABLE delivery_tasks
ADD COLUMN IF NOT EXISTS delivery_location_lng DOUBLE PRECISION;

CREATE INDEX IF NOT EXISTS idx_delivery_tasks_provider_delivery
ON delivery_tasks(organization_id, provider, provider_delivery_id);
