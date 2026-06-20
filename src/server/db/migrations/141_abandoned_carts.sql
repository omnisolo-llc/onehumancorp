-- +goose Up
CREATE TABLE IF NOT EXISTS abandoned_carts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    cart_id TEXT NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    customer_email TEXT,
    customer_phone TEXT,
    items JSONB,
    status TEXT NOT NULL DEFAULT 'PENDING',
    abandoned_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('abandoned_carts') IS NOT NULL THEN
        ALTER TABLE abandoned_carts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_abandoned_carts ON abandoned_carts
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_abandoned_carts ON abandoned_carts;
END
$$;

DROP TABLE IF EXISTS abandoned_carts CASCADE;
