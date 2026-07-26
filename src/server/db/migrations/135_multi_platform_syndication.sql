-- Migration 135: Multi-Platform Product Syndication Engine

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'sync_status_enum') THEN
        CREATE TYPE sync_status_enum AS ENUM ('PENDING', 'ACTIVE', 'FAILED');
    END IF;
END$$;

CREATE TABLE IF NOT EXISTS platform_listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    platform_id TEXT NOT NULL,
    platform_external_id TEXT,
    optimized_title TEXT,
    optimized_description TEXT,
    sync_status sync_status_enum NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_platform_listings_tenant_id ON platform_listings(tenant_id);
CREATE INDEX IF NOT EXISTS idx_platform_listings_product_id ON platform_listings(product_id);
CREATE INDEX IF NOT EXISTS idx_platform_listings_platform_id ON platform_listings(platform_id);

ALTER TABLE platform_listings ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON platform_listings
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS inventory_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    delta INTEGER NOT NULL,
    source TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_inventory_ledger_tenant_id ON inventory_ledger(tenant_id);
CREATE INDEX IF NOT EXISTS idx_inventory_ledger_product_id ON inventory_ledger(product_id);

ALTER TABLE inventory_ledger ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON inventory_ledger
    USING (tenant_id = current_setting('app.current_tenant', true));
