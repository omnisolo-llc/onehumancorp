CREATE TABLE IF NOT EXISTS smart_pricing_policies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    min_margin_percent DOUBLE PRECISION NOT NULL,
    auto_discount_trigger_days_stagnant INTEGER NOT NULL,
    max_discount_percent DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, product_id)
);
ALTER TABLE smart_pricing_policies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_smart_pricing_policies ON smart_pricing_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS active_discounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    policy_id TEXT NOT NULL REFERENCES smart_pricing_policies(id) ON DELETE CASCADE,
    discount_amount DOUBLE PRECISION NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE active_discounts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_active_discounts ON active_discounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
