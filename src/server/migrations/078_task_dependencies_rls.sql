-- Add tenant_id to task_dependencies if missing, and enable RLS
ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_task_dependencies_tenant_id ON task_dependencies(tenant_id);

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
