-- +goose Up
-- Migration 131: Enforce Missing Row Level Security Policies for specific tables

DO $$
BEGIN
    IF to_regclass('business_milestones') IS NOT NULL THEN
        ALTER TABLE IF EXISTS business_milestones ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies
            WHERE schemaname = current_schema()
              AND tablename = 'business_milestones'
              AND policyname = 'tenant_isolation_business_milestones'
        ) THEN
            CREATE POLICY tenant_isolation_business_milestones ON business_milestones
                USING (tenant_id::text = current_setting('app.current_tenant', true))
                WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END $$;

DO $$
BEGIN
    IF to_regclass('shared_tasks_decomposition') IS NOT NULL THEN
        ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies
            WHERE schemaname = current_schema()
              AND tablename = 'shared_tasks_decomposition'
              AND policyname = 'tenant_isolation_shared_tasks_decomposition'
        ) THEN
            CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition
                USING (organization_id::text = current_setting('app.current_tenant', true))
                WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END $$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('business_milestones') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_business_milestones ON business_milestones;
        ALTER TABLE IF EXISTS business_milestones DISABLE ROW LEVEL SECURITY;
    END IF;
END $$;

DO $$
BEGIN
    IF to_regclass('shared_tasks_decomposition') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
        ALTER TABLE IF EXISTS shared_tasks_decomposition DISABLE ROW LEVEL SECURITY;
    END IF;
END $$;
