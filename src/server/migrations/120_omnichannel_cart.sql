-- +goose Up
-- Migration 120: Omnichannel Cart Architecture

CREATE TABLE IF NOT EXISTS carts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL DEFAULT 'online', -- 'online' or 'in_store'
    status TEXT NOT NULL DEFAULT 'active', -- 'active', 'pending_payment', 'completed', 'abandoned'
    total_amount_cents BIGINT DEFAULT 0,
    currency TEXT DEFAULT 'usd',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS cart_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    cart_id TEXT NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL,
    variant_id TEXT,
    quantity INT NOT NULL DEFAULT 1,
    unit_price_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('carts') IS NOT NULL THEN
        ALTER TABLE carts ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'carts'
                AND policyname = 'tenant_isolation_carts'
        ) THEN
            CREATE POLICY tenant_isolation_carts ON carts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('cart_items') IS NOT NULL THEN
        ALTER TABLE cart_items ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'cart_items'
                AND policyname = 'tenant_isolation_cart_items'
        ) THEN
            CREATE POLICY tenant_isolation_cart_items ON cart_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;
