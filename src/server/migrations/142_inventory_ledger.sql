-- +goose Up
-- Migration 142: Add inventory_ledger table for fine-grained stock tracking

CREATE TABLE IF NOT EXISTS inventory_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    change_amount INT NOT NULL,
    reason TEXT NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inventory_ledger_tenant_product ON inventory_ledger(tenant_id, product_id);
CREATE INDEX IF NOT EXISTS idx_inventory_ledger_created_at ON inventory_ledger(created_at DESC);

CREATE TABLE IF NOT EXISTS in_person_orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    terminal_session_id TEXT REFERENCES pos_terminal_sessions(id),
    device_id TEXT,
    cashier_id TEXT,
    payment_method TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    -- inventory_ledger RLS
    IF to_regclass('inventory_ledger') IS NOT NULL THEN
        ALTER TABLE inventory_ledger ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE tablename = 'inventory_ledger' AND policyname = 'tenant_isolation_inventory_ledger'
        ) THEN
            CREATE POLICY tenant_isolation_inventory_ledger ON inventory_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    -- in_person_orders RLS
    IF to_regclass('in_person_orders') IS NOT NULL THEN
        ALTER TABLE in_person_orders ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE tablename = 'in_person_orders' AND policyname = 'tenant_isolation_in_person_orders'
        ) THEN
            CREATE POLICY tenant_isolation_in_person_orders ON in_person_orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
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
    IF to_regclass('in_person_orders') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_in_person_orders ON in_person_orders;
        ALTER TABLE in_person_orders DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS inventory_ledger CASCADE;
DROP TABLE IF EXISTS in_person_orders CASCADE;
