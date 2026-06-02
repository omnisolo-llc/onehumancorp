-- Migration 066: Fix missing explicit RLS on specific tables

ALTER TABLE builder_brand_toolboxes ENABLE ROW LEVEL SECURITY;
ALTER TABLE migration_jobs ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_builder_brand_toolboxes' AND tablename = 'builder_brand_toolboxes') THEN
        CREATE POLICY tenant_isolation_builder_brand_toolboxes ON builder_brand_toolboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_migration_jobs' AND tablename = 'migration_jobs') THEN
        CREATE POLICY tenant_isolation_migration_jobs ON migration_jobs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
