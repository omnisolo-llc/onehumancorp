-- Add RLS to consolidated_memory
ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'consolidated_memory'
          AND policyname = 'tenant_isolation_consolidated_memory'
    ) THEN
        CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;
