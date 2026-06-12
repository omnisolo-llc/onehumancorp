CREATE TABLE IF NOT EXISTS pricing_rules (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    service_category TEXT NOT NULL,
    rule_name TEXT NOT NULL,
    base_price_cents BIGINT NOT NULL,
    modifiers JSONB NOT NULL DEFAULT '[]', -- E.g., [{"type": "flat", "condition": "rush", "value_cents": 5000}, {"type": "percentage", "condition": "weekend", "value": 20}]
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE pricing_rules ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_pricing_rules ON pricing_rules
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_pricing_rules_tenant ON pricing_rules(tenant_id);
