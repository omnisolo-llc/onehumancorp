-- +goose Up
CREATE TABLE IF NOT EXISTS customer_aliases (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    channel_type TEXT NOT NULL,
    identifier TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, channel_type, identifier)
);

ALTER TABLE customer_aliases ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_aliases ON customer_aliases USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_customer_aliases ON customer_aliases;
DROP TABLE IF EXISTS customer_aliases CASCADE;
