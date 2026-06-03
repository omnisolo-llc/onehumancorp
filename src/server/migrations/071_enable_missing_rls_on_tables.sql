-- Enable Row Level Security (RLS) on tables containing multi-tenant data

-- Found in 014_shared_tasks.sql
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'shared_tasks'
            AND policyname = 'tenant_isolation_shared_tasks'
    ) THEN
        CREATE POLICY "tenant_isolation_shared_tasks" ON shared_tasks FOR ALL USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Found in 017_business_milestones.sql
ALTER TABLE IF EXISTS business_milestones ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'business_milestones'
            AND policyname = 'tenant_isolation_business_milestones'
    ) THEN
        CREATE POLICY "tenant_isolation_business_milestones" ON business_milestones FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Found in 058_shared_tasks_decomposition_table.sql
ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'shared_tasks_decomposition'
            AND policyname = 'tenant_isolation_shared_tasks_decomposition'
    ) THEN
        CREATE POLICY "tenant_isolation_shared_tasks_decomposition" ON shared_tasks_decomposition FOR ALL USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
