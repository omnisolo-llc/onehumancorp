-- +goose Up
CREATE TABLE IF NOT EXISTS offers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    type TEXT NOT NULL, -- 'product', 'service', 'digital'
    title TEXT NOT NULL,
    description TEXT,
    price_cents BIGINT DEFAULT 0,
    currency TEXT DEFAULT 'USD',
    inventory_count INT DEFAULT 0,
    locked_quantity INT DEFAULT 0,
    available_quantity INT DEFAULT 0,
    duration_minutes INT DEFAULT 60,
    metadata JSONB DEFAULT '{}',
    is_subscribable BOOLEAN DEFAULT false,
    subscription_frequency TEXT,
    subscription_discount_percent INT,
    seo_title TEXT,
    seo_description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_offers_tenant_id ON offers(tenant_id);

ALTER TABLE offers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_offers ON offers;
CREATE POLICY tenant_isolation_offers
ON offers
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_offers ON offers;
DROP TABLE IF EXISTS offers CASCADE;
