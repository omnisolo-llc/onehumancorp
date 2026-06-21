-- +goose Up
CREATE TABLE IF NOT EXISTS suppliers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('suppliers') IS NOT NULL THEN
        ALTER TABLE suppliers ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'suppliers'
                AND policyname = 'tenant_isolation_suppliers'
        ) THEN
            CREATE POLICY tenant_isolation_suppliers ON suppliers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        ADD COLUMN IF NOT EXISTS low_stock_threshold INT DEFAULT 10,
        ADD COLUMN IF NOT EXISTS supplier_id TEXT REFERENCES suppliers(id) ON DELETE SET NULL;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        DROP COLUMN IF NOT EXISTS low_stock_threshold,
        DROP COLUMN IF NOT EXISTS supplier_id;
    END IF;

    IF to_regclass('suppliers') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_suppliers ON suppliers;
        ALTER TABLE suppliers DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS suppliers CASCADE;
