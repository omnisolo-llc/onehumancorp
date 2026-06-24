-- +goose Up
-- Migration 156: Add inventory_reservations table

CREATE TABLE IF NOT EXISTS inventory_reservations (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inventory_reservations_tenant_id ON inventory_reservations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_inventory_reservations_product_id ON inventory_reservations(product_id);

ALTER TABLE inventory_reservations ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'inventory_reservations'
            AND policyname = 'tenant_isolation_inventory_reservations'
    ) THEN
        CREATE POLICY tenant_isolation_inventory_reservations ON inventory_reservations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_inventory_reservations ON inventory_reservations;
END
$$;

DROP TABLE IF EXISTS inventory_reservations CASCADE;
