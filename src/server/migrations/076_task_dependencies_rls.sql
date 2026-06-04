-- Add missing tenant_id to task_dependencies
ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_task_dependencies_tenant_id ON task_dependencies(tenant_id);

-- Enable RLS for task_dependencies
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'task_dependencies'
          AND policyname = 'tenant_isolation_task_dependencies'
    ) THEN
        CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Add missing organization_id to shared_task_dependencies
ALTER TABLE shared_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_shared_task_dependencies_org_id ON shared_task_dependencies(organization_id);

-- Enable RLS for shared_task_dependencies
ALTER TABLE shared_task_dependencies ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'shared_task_dependencies'
          AND policyname = 'tenant_isolation_shared_task_dependencies'
    ) THEN
        CREATE POLICY tenant_isolation_shared_task_dependencies ON shared_task_dependencies
            USING (organization_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
