CREATE TABLE IF NOT EXISTS fulfillment_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'PENDING',
    next_fulfillment_date DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_tenant_id ON fulfillment_schedules(tenant_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_customer_id ON fulfillment_schedules(customer_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_date ON fulfillment_schedules(next_fulfillment_date);

ALTER TABLE fulfillment_schedules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_schedules ON fulfillment_schedules;
CREATE POLICY tenant_isolation_fulfillment_schedules
ON fulfillment_schedules
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
