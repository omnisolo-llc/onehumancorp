-- Unified Hybrid Catalog Migration
-- Resolves #9527: Implement Unified Hybrid Catalog System for Products and Services

-- 1. Create Catalog Items Table (Hybrid for physical, digital, and service)
CREATE TABLE IF NOT EXISTS catalog_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    organization_id TEXT NOT NULL, -- Logical link, tenant_id used for isolation
    name TEXT NOT NULL,
    description TEXT,
    type TEXT NOT NULL, -- 'physical', 'digital', 'service'
    price_cents BIGINT DEFAULT 0,
    currency TEXT DEFAULT 'USD',
    duration_minutes INT, -- Specific for 'service'
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 2. Create Tenant Calendars
CREATE TABLE IF NOT EXISTS tenant_calendars (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    organization_id TEXT NOT NULL,
    timezone TEXT DEFAULT 'UTC',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 3. Enable RLS and add isolation policies
ALTER TABLE catalog_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_catalog_items ON catalog_items
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tenant_calendars ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tenant_calendars ON tenant_calendars
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 4. Initial migration of existing products to catalog_items
INSERT INTO catalog_items (id, tenant_id, organization_id, name, description, type, price_cents, currency, metadata, created_at, updated_at)
SELECT id, tenant_id, organization_id, name, description, COALESCE(type, 'physical'), price_cents, currency, metadata, created_at, updated_at
FROM products;

-- 5. Deprecation Notice
-- The 'products' table is now deprecated in favor of 'catalog_items'.
-- Data is being migrated. Applications should switch to 'catalog_items'.
-- The 'products' table will be dropped in a future release once all consumers are updated.
