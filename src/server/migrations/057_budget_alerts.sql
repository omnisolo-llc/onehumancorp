CREATE TABLE IF NOT EXISTS tenant_budget_alerts (
    tenant_id TEXT PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    threshold_usd DOUBLE PRECISION NOT NULL DEFAULT 100.0,
    notify_at_pct DOUBLE PRECISION NOT NULL DEFAULT 80.0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE tenant_budget_alerts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tenant_budget_alerts ON tenant_budget_alerts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
