-- +goose Up
-- Migration 035: Add storefront_edge_cache table

CREATE TABLE IF NOT EXISTS storefront_edge_cache (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    route_key TEXT NOT NULL,
    html_content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, route_key)
);

DO $$
BEGIN
    IF to_regclass('storefront_edge_cache') IS NOT NULL THEN
        ALTER TABLE storefront_edge_cache ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'storefront_edge_cache'
                AND policyname = 'tenant_isolation_storefront_edge_cache'
        ) THEN
            CREATE POLICY tenant_isolation_storefront_edge_cache ON storefront_edge_cache USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('storefront_edge_cache') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_storefront_edge_cache ON storefront_edge_cache;
        ALTER TABLE storefront_edge_cache DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS storefront_edge_cache CASCADE;
