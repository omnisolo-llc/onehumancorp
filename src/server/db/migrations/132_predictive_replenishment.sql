CREATE TABLE IF NOT EXISTS fulfillment_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    last_fulfillment_date TIMESTAMPTZ,
    predicted_next_fulfillment_date TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_tenant ON fulfillment_schedules(tenant_id);
ALTER TABLE fulfillment_schedules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_schedules ON fulfillment_schedules;
CREATE POLICY tenant_isolation_fulfillment_schedules
ON fulfillment_schedules
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
