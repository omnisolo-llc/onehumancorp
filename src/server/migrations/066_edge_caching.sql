-- Edge Caching Dynamic Storefront Engine
-- GitHub Issue #22709

CREATE TABLE IF NOT EXISTS ohc_storefront_cache_config (
    tenant_id TEXT PRIMARY KEY,
    s_maxage INTEGER NOT NULL DEFAULT 60,
    stale_while_revalidate INTEGER NOT NULL DEFAULT 86400,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE ohc_storefront_cache_config ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_storefront_cache_config ON ohc_storefront_cache_config;
CREATE POLICY tenant_isolation_ohc_storefront_cache_config
ON ohc_storefront_cache_config
USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_edge_asset (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    cache_key TEXT NOT NULL,
    tags JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_edge_asset_tenant
ON ohc_edge_asset(tenant_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ohc_edge_asset_cache_key
ON ohc_edge_asset(tenant_id, cache_key);

ALTER TABLE ohc_edge_asset ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_edge_asset ON ohc_edge_asset;
CREATE POLICY tenant_isolation_ohc_edge_asset
ON ohc_edge_asset
USING (tenant_id = current_setting('app.current_tenant', true));
