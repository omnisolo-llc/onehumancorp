-- Smart Pricing Policies & Active Discounts
-- GitHub Issue #24725

CREATE TABLE IF NOT EXISTS smart_pricing_policies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    min_margin_percent DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    auto_discount_trigger_days_stagnant INTEGER NOT NULL DEFAULT 30,
    max_discount_percent DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_smart_pricing_policies_tenant_id ON smart_pricing_policies(tenant_id);
CREATE INDEX IF NOT EXISTS idx_smart_pricing_policies_product_id ON smart_pricing_policies(product_id);

ALTER TABLE smart_pricing_policies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_smart_pricing_policies ON smart_pricing_policies;
CREATE POLICY tenant_isolation_smart_pricing_policies
ON smart_pricing_policies
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS active_discounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    policy_id TEXT NOT NULL REFERENCES smart_pricing_policies(id) ON DELETE CASCADE,
    discount_amount DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_active_discounts_tenant_id ON active_discounts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_active_discounts_policy_id ON active_discounts(policy_id);

ALTER TABLE active_discounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_active_discounts ON active_discounts;
CREATE POLICY tenant_isolation_active_discounts
ON active_discounts
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
