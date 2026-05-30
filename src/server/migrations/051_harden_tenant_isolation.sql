-- Migration 051: Harden Tenant Isolation for missing tables

-- 1. Epics and Tasks (from 021_epics_tasks.sql)
ALTER TABLE epics ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;

-- 2. Telemetry Buffer
ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id TEXT;

-- 3. Task Dependencies
ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;

-- Enable RLS and Create Policies
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'epics',
            'tasks',
            'business_milestones',
            'telemetry_buffer',
            'task_dependencies'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

            pol_name := format('tenant_isolation_%s', t_name);
            IF NOT EXISTS (
                SELECT 1
                FROM pg_policies
                WHERE schemaname = current_schema()
                    AND tablename = t_name
                    AND policyname = pol_name
            ) THEN
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        END IF;
    END LOOP;
END
$$;

-- Special handling for legacy_tasks if it lost its policy or needs a refresh
DO $$
BEGIN
    IF to_regclass('legacy_tasks') IS NOT NULL THEN
        ALTER TABLE legacy_tasks ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE tablename = 'legacy_tasks' AND policyname = 'tenant_isolation_legacy_tasks'
        ) THEN
            CREATE POLICY tenant_isolation_legacy_tasks ON legacy_tasks
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;
