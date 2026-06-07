-- Migration 105: Harden Row Level Security and System Elevation
-- Systematically enforce RLS on all tables that should be tenant-isolated.

DO $$
DECLARE
    t_name text;
    pol_name text;
    col_type text;
BEGIN
    FOR t_name IN
        SELECT tablename
        FROM pg_tables
        WHERE schemaname = 'public'
    LOOP
        -- Skip system tables or tables we know should be global (if any)
        IF t_name IN ('migrations', 'spatial_ref_sys', 'pg_stat_statements') THEN
            CONTINUE;
        END IF;

        -- Target tables with tenant_id or organization_id
        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name=t_name AND column_name='tenant_id') THEN
            SELECT data_type INTO col_type FROM information_schema.columns WHERE table_name=t_name AND column_name='tenant_id';

            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
            EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY', t_name);

            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);

            IF col_type = 'uuid' THEN
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (tenant_id = current_setting(''app.current_tenant'', true)::uuid) WITH CHECK (tenant_id = current_setting(''app.current_tenant'', true)::uuid)',
                    pol_name,
                    t_name
                );
            ELSE
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        ELSIF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name=t_name AND column_name='organization_id') THEN
            SELECT data_type INTO col_type FROM information_schema.columns WHERE table_name=t_name AND column_name='organization_id';

            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
            EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY', t_name);

            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);

            IF col_type = 'uuid' THEN
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (organization_id = current_setting(''app.current_tenant'', true)::uuid) WITH CHECK (organization_id = current_setting(''app.current_tenant'', true)::uuid)',
                    pol_name,
                    t_name
                );
            ELSE
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        END IF;
    END LOOP;
END
$$;

-- Create the ohc_bypassrls role if it doesn't exist (used for system-level background tasks)
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN
        CREATE ROLE ohc_bypassrls NOLOGIN;
        -- We do NOT give it BYPASSRLS directly here, it should be granted explicitly in the environment
        -- or used via SET ROLE if the application user has the right to assume it.
    END IF;
END
$$;
