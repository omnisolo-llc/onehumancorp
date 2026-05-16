-- Ensure consolidated_memory RLS is active
ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS consolidated_memory FORCE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_consolidated_memory' AND tablename = 'consolidated_memory'
    ) THEN
        CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
