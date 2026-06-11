CREATE TABLE IF NOT EXISTS pricing_rules (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT,
    condition_type VARCHAR NOT NULL,
    discount_percent DOUBLE PRECISION NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_pricing_rules_tenant ON pricing_rules(tenant_id);

ALTER TABLE pricing_rules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_rules ON pricing_rules;
CREATE POLICY tenant_isolation_pricing_rules ON pricing_rules
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS targeted_discounts (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    product_id VARCHAR NOT NULL,
    customer_segment VARCHAR NOT NULL,
    discount_percent DOUBLE PRECISION NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_targeted_discounts_tenant ON targeted_discounts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_targeted_discounts_product ON targeted_discounts(product_id);

ALTER TABLE targeted_discounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_targeted_discounts ON targeted_discounts;
CREATE POLICY tenant_isolation_targeted_discounts ON targeted_discounts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS price_recommendations (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    product_id VARCHAR NOT NULL,
    suggestion_text TEXT NOT NULL,
    discount_percent DOUBLE PRECISION NOT NULL,
    target_segment VARCHAR,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_price_recommendations_tenant ON price_recommendations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_price_recommendations_status ON price_recommendations(status);

ALTER TABLE price_recommendations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_price_recommendations ON price_recommendations;
CREATE POLICY tenant_isolation_price_recommendations ON price_recommendations
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
