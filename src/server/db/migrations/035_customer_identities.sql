-- +goose Up
-- Migration 035: Add customer_identities table

CREATE TABLE IF NOT EXISTS customer_identities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    channel TEXT NOT NULL,
    channel_identity TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, channel, channel_identity)
);

DO $$
BEGIN
    IF to_regclass('customer_identities') IS NOT NULL THEN
        ALTER TABLE customer_identities ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_customer_identities ON customer_identities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
END
$$;

DROP TABLE IF EXISTS customer_identities CASCADE;
