CREATE TABLE IF NOT EXISTS tenant_ai_budgets (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    year_month TEXT NOT NULL,
    actions_used INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, year_month)
);
ALTER TABLE tenant_ai_budgets ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tenant_ai_budgets ON tenant_ai_budgets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
