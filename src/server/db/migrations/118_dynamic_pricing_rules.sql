CREATE TABLE IF NOT EXISTS dynamic_pricing_rules (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    rule_name TEXT NOT NULL,
    condition_variable TEXT NOT NULL, -- e.g., 'rush', 'distance', 'customer_tier'
    condition_operator TEXT NOT NULL, -- e.g., 'equals', 'greater_than'
    condition_value TEXT NOT NULL,
    adjustment_type TEXT NOT NULL, -- e.g., 'flat', 'percentage'
    adjustment_amount DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE dynamic_pricing_rules ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_dynamic_pricing_rules ON dynamic_pricing_rules
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
