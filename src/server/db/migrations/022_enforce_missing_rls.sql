-- +goose Up

-- Fix ohc_collective_loyalty_balance
ALTER TABLE ohc_collective_loyalty_balance ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
CREATE INDEX IF NOT EXISTS idx_ohc_collective_loyalty_balance_tenant_id ON ohc_collective_loyalty_balance(tenant_id);

ALTER TABLE ohc_collective_loyalty_balance ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'ohc_collective_loyalty_balance'
          AND policyname = 'tenant_isolation_ohc_collective_loyalty_balance'
    ) THEN
        CREATE POLICY tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
-- Revert ohc_collective_loyalty_balance
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance;
ALTER TABLE ohc_collective_loyalty_balance DISABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_collective_loyalty_balance DROP COLUMN IF EXISTS tenant_id;
