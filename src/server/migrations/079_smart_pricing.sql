-- Smart Pricing Engine Migrations

CREATE TABLE IF NOT EXISTS smart_pricing_policies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    min_margin_percent DECIMAL NOT NULL DEFAULT 0.0,
    auto_discount_trigger_days_stagnant INTEGER NOT NULL DEFAULT 30,
    max_discount_percent DECIMAL NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_smart_pricing_policies_tenant ON smart_pricing_policies(tenant_id);
CREATE INDEX IF NOT EXISTS idx_smart_pricing_policies_product ON smart_pricing_policies(product_id);

ALTER TABLE smart_pricing_policies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_smart_pricing_policies ON smart_pricing_policies;
CREATE POLICY tenant_isolation_smart_pricing_policies
ON smart_pricing_policies
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS active_discounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    policy_id TEXT NOT NULL REFERENCES smart_pricing_policies(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    discount_amount DECIMAL NOT NULL DEFAULT 0.0,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_active_discounts_tenant ON active_discounts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_active_discounts_product ON active_discounts(product_id);

ALTER TABLE active_discounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_active_discounts ON active_discounts;
CREATE POLICY tenant_isolation_active_discounts
ON active_discounts
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
