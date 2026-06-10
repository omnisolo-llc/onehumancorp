-- +goose Up
-- Migration 115: Add customer_identities table for omnichannel identity resolution

CREATE TABLE IF NOT EXISTS customer_identities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    channel TEXT NOT NULL, -- e.g., 'instagram', 'whatsapp', 'email', 'phone'
    identifier TEXT NOT NULL, -- e.g., handle, phone number, email address
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, channel, identifier)
);

CREATE INDEX IF NOT EXISTS idx_customer_identities_lookup ON customer_identities(tenant_id, channel, identifier);

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
