-- 076_tenant_isolation_audit.sql
-- Final sweep to ensure absolutely NO data leaks between tenants in Cloud Mode.
-- Ensure strict bound RLS using current_setting('app.current_tenant', true).

DO $$
DECLARE
    t_name text;
BEGIN
    FOR t_name IN
        SELECT table_name
        FROM information_schema.columns
        WHERE table_schema = 'public'
        AND column_name IN ('tenant_id', 'organization_id')
    LOOP
        EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY;', t_name);
    END LOOP;
END $$;

-- Drop all permissive policies or ones with missing 'strict' suffix
DO $$
DECLARE
    pol RECORD;
BEGIN
    FOR pol IN
        SELECT policyname, tablename
        FROM pg_policies
        WHERE schemaname = 'public'
        AND (policyname LIKE '%_t' OR policyname NOT LIKE '%_strict')
    LOOP
        -- Don't drop Postgres internal policies if any
        EXECUTE format('DROP POLICY IF EXISTS %I ON %I;', pol.policyname, pol.tablename);
    END LOOP;
END $$;

-- Re-create strict policies for tables that have tenant_id
DO $$
DECLARE
    t_name text;
BEGIN
    FOR t_name IN
        SELECT table_name
        FROM information_schema.columns
        WHERE table_schema = 'public'
        AND column_name = 'tenant_id'
        AND table_name != 'tenants' -- handled separately
    LOOP
        -- create or replace
        EXECUTE format('DROP POLICY IF EXISTS tenant_isolation_%I_strict ON %I;', t_name, t_name);
        EXECUTE format('CREATE POLICY tenant_isolation_%I_strict ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true));', t_name, t_name);
    END LOOP;
END $$;

-- Handle organization_id for those missing tenant_id (or as fallback)
DO $$
DECLARE
    t_name text;
BEGIN
    FOR t_name IN
        SELECT c1.table_name
        FROM information_schema.columns c1
        LEFT JOIN information_schema.columns c2
          ON c1.table_name = c2.table_name AND c2.column_name = 'tenant_id'
        WHERE c1.table_schema = 'public'
        AND c1.column_name = 'organization_id'
        AND c2.column_name IS NULL
    LOOP
        EXECUTE format('DROP POLICY IF EXISTS tenant_isolation_%I_org_strict ON %I;', t_name, t_name);
        EXECUTE format('CREATE POLICY tenant_isolation_%I_org_strict ON %I USING (organization_id::text = current_setting(''app.current_tenant'', true));', t_name, t_name);
    END LOOP;
END $$;

-- Special cases
DROP POLICY IF EXISTS tenant_isolation_tenants_strict ON tenants;
CREATE POLICY tenant_isolation_tenants_strict ON tenants USING (id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_task_dependencies_strict ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies_strict ON task_dependencies USING (task_id::text IN (SELECT id::text FROM shared_tasks WHERE tenant_id::text = current_setting('app.current_tenant', true)));
