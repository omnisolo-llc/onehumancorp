-- Migration 055: Fix Missing Row Level Security on various tables

-- Add tenant_id to tables missing it
ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_task_dependencies_tenant_id ON task_dependencies(tenant_id);

ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_telemetry_buffer_tenant_id ON telemetry_buffer(tenant_id);

ALTER TABLE epics ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_epics_tenant_id ON epics(tenant_id);

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_tasks_tenant_id ON tasks(tenant_id);

ALTER TABLE embedding_cache ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_embedding_cache_tenant_id ON embedding_cache(tenant_id);

-- Enable RLS
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
ALTER TABLE business_milestones ENABLE ROW LEVEL SECURITY;
ALTER TABLE epics ENABLE ROW LEVEL SECURITY;
ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;

-- Define RLS Policies
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'telemetry_buffer',
            'business_milestones',
            'epics',
            'tasks',
            'embedding_cache',
            'task_dependencies'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
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

-- Enable RLS on shared_task_dependencies (using organization_id from shared_tasks since this relates to shared_tasks)
-- However, it does not have tenant_id directly. We should add organization_id.
ALTER TABLE shared_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_shared_task_dependencies_org_id ON shared_task_dependencies(organization_id);

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
