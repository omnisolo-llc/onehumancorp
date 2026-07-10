-- +goose Up
-- Migration 210: Centralized Inventory Ledger

CREATE TABLE IF NOT EXISTS inventory_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    location_id TEXT DEFAULT 'default',
    type TEXT NOT NULL, -- e.g., 'sale', 'restock', 'return', 'manual_adjustment'
    quantity_change INT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('inventory_transactions') IS NOT NULL THEN
        ALTER TABLE inventory_transactions ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'inventory_transactions'
                AND policyname = 'tenant_isolation_inventory_transactions'
        ) THEN
            CREATE POLICY tenant_isolation_inventory_transactions ON inventory_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- Ensure inventory_levels has the required columns
ALTER TABLE inventory_levels ADD COLUMN IF NOT EXISTS available_count INT DEFAULT 0;
ALTER TABLE inventory_levels ADD COLUMN IF NOT EXISTS committed_count INT DEFAULT 0;

-- Backfill available_count and committed_count from products if they are 0
UPDATE inventory_levels il
SET available_count = p.available_quantity,
    committed_count = p.locked_quantity
FROM products p
WHERE il.product_id = p.id AND il.tenant_id = p.tenant_id AND il.available_count = 0;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('inventory_transactions') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_inventory_transactions ON inventory_transactions;
        ALTER TABLE inventory_transactions DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS inventory_transactions CASCADE;

ALTER TABLE inventory_levels DROP COLUMN IF EXISTS available_count;
ALTER TABLE inventory_levels DROP COLUMN IF EXISTS committed_count;
