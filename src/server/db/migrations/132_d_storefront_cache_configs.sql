-- Migration 132_d: Storefront Cache Configs
CREATE TABLE IF NOT EXISTS storefront_cache_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    cache_duration_seconds INTEGER NOT NULL DEFAULT 3600,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_storefront_cache_configs_tenant_id ON storefront_cache_configs(tenant_id);

ALTER TABLE storefront_cache_configs ENABLE ROW LEVEL SECURITY;
ALTER TABLE storefront_cache_configs FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_storefront_cache_configs ON storefront_cache_configs
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
