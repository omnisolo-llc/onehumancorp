-- Migration 079: Agentic Smart Pricing & Dynamic Discount Engine

CREATE TABLE IF NOT EXISTS smart_pricing_policies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    min_margin_percent DECIMAL NOT NULL,
    auto_discount_trigger_days_stagnant INT NOT NULL,
    max_discount_percent DECIMAL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS active_discounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    policy_id TEXT REFERENCES smart_pricing_policies(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    discount_amount DECIMAL NOT NULL,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE smart_pricing_policies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_smart_pricing_policies ON smart_pricing_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE active_discounts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_active_discounts ON active_discounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
