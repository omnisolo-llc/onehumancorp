-- Enable RLS and setup tenant isolation policies for swarm_tasks and task_dependencies if missing
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = 'swarm_tasks' AND rowsecurity = true
    ) THEN
        ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'swarm_tasks'
          AND policyname = 'tenant_isolation_swarm_tasks'
    ) THEN
        CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks
            USING (tenant_id = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = 'task_dependencies' AND rowsecurity = true
    ) THEN
        ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
    END IF;

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
END $$;
