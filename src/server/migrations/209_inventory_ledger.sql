-- +goose Up
CREATE TABLE IF NOT EXISTS inventory_ledger (
    ledger_id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id UUID NOT NULL,
    quantity_delta INT NOT NULL,
    transaction_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('inventory_ledger') IS NOT NULL THEN
        ALTER TABLE inventory_ledger ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'inventory_ledger'
                AND policyname = 'tenant_isolation_inventory_ledger'
        ) THEN
            CREATE POLICY tenant_isolation_inventory_ledger ON inventory_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('inventory_ledger') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_inventory_ledger ON inventory_ledger;
        ALTER TABLE inventory_ledger DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS inventory_ledger CASCADE;
