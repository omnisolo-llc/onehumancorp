-- 079_smart_pricing.sql
CREATE TABLE IF NOT EXISTS smart_pricing_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    product_id UUID NOT NULL,
    min_margin_percent DOUBLE PRECISION NOT NULL,
    auto_discount_trigger_days_stagnant INTEGER NOT NULL,
    max_discount_percent DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE smart_pricing_policies ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_smart_pricing_policies ON smart_pricing_policies
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS active_discounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_id UUID NOT NULL REFERENCES smart_pricing_policies(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    discount_amount DOUBLE PRECISION NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE active_discounts ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_active_discounts ON active_discounts
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);
