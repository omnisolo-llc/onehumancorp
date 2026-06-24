-- +goose Up
CREATE TABLE IF NOT EXISTS tenant_feed_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    action_type TEXT NOT NULL,
    action_payload JSONB,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('tenant_feed_items') IS NOT NULL THEN
        ALTER TABLE tenant_feed_items ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_tenant_feed_items ON tenant_feed_items;
        CREATE POLICY tenant_isolation_tenant_feed_items ON tenant_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_tenant_feed_items ON tenant_feed_items;
END
$$;

DROP TABLE IF EXISTS tenant_feed_items CASCADE;
