-- +goose Up
-- Migration 127: Identity Links

CREATE TABLE IF NOT EXISTS identity_links (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    external_id TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, channel, external_id)
);

DO $$
BEGIN
    IF to_regclass('identity_links') IS NOT NULL THEN
        ALTER TABLE identity_links ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_identity_links ON identity_links
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_identity_links ON identity_links;
END
$$;

DROP TABLE IF EXISTS identity_links CASCADE;
