CREATE TABLE IF NOT EXISTS dynamic_pricing_rules (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    base_price_cents BIGINT NOT NULL,
    rules JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dynamic_pricing_rules_tenant ON dynamic_pricing_rules(tenant_id);
CREATE INDEX IF NOT EXISTS idx_dynamic_pricing_rules_product ON dynamic_pricing_rules(product_id);

ALTER TABLE dynamic_pricing_rules ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_dynamic_pricing_rules ON dynamic_pricing_rules;
CREATE POLICY tenant_isolation_dynamic_pricing_rules ON dynamic_pricing_rules
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
