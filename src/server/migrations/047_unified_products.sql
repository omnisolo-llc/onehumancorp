-- 047_unified_products.sql
-- Unified Polymorphic Product Model for OHC.

CREATE TABLE IF NOT EXISTS products (
    id                   TEXT PRIMARY KEY,
    organization_id      TEXT NOT NULL,
    name                 TEXT NOT NULL,
    description          TEXT,
    price_cents          BIGINT NOT NULL DEFAULT 0,
    currency             TEXT NOT NULL DEFAULT 'USD',
    fulfillment_strategy TEXT NOT NULL, -- 'physical', 'digital', 'booking'
    metadata             JSONB DEFAULT '{}',
    created_at           TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_products_org ON products (organization_id);
CREATE INDEX idx_products_fulfillment ON products (fulfillment_strategy);

-- Enable RLS for multi-tenant isolation
ALTER TABLE products ENABLE ROW LEVEL SECURITY;
