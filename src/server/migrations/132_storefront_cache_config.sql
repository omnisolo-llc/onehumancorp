CREATE TABLE IF NOT EXISTS storefront_cache_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    edge_caching_enabled BOOLEAN NOT NULL DEFAULT true,
    auto_seo_prerender BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT storefront_cache_configs_tenant_id_key UNIQUE (tenant_id)
);

CREATE INDEX IF NOT EXISTS idx_storefront_cache_configs_tenant_id ON storefront_cache_configs(tenant_id);

ALTER TABLE storefront_cache_configs ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON storefront_cache_configs
    USING (tenant_id = current_setting('app.current_tenant', true));
