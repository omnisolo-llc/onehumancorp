CREATE TABLE IF NOT EXISTS smart_pricing_policies (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    product_id UUID NOT NULL,
    min_margin_percent DOUBLE PRECISION NOT NULL,
    auto_discount_trigger_days_stagnant INTEGER NOT NULL,
    max_discount_percent DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_smart_pricing_policies_tenant ON smart_pricing_policies(tenant_id);
CREATE INDEX IF NOT EXISTS idx_smart_pricing_policies_product ON smart_pricing_policies(product_id);

ALTER TABLE smart_pricing_policies ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_smart_pricing_policies ON smart_pricing_policies;
CREATE POLICY tenant_isolation_smart_pricing_policies ON smart_pricing_policies
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS active_discounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    policy_id UUID REFERENCES smart_pricing_policies(id) ON DELETE CASCADE,
    product_id UUID NOT NULL,
    discount_amount DOUBLE PRECISION NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_active_discounts_tenant ON active_discounts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_active_discounts_product ON active_discounts(product_id);
CREATE INDEX IF NOT EXISTS idx_active_discounts_policy ON active_discounts(policy_id);

ALTER TABLE active_discounts ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_active_discounts ON active_discounts;
CREATE POLICY tenant_isolation_active_discounts ON active_discounts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
