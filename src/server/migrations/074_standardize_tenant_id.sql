-- 074_standardize_tenant_id.sql
-- This migration standardizes the column name for tenant isolation to 'tenant_id' across all tables.
-- It renames 'organization_id' to 'tenant_id' where it exists and updates RLS policies.

DO $$
DECLARE
    r RECORD;
BEGIN
    -- Rename organization_id to tenant_id in all tables where it exists
    FOR r IN
        SELECT table_name, column_name
        FROM information_schema.columns
        WHERE column_name = 'organization_id'
        AND table_schema = 'public'
    LOOP
        -- Special case: if both exist, we might want to drop organization_id if it's redundant
        -- For onboarding_state, it seems to have both.
        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = r.table_name AND column_name = 'tenant_id' AND table_schema = 'public') THEN
            EXECUTE 'ALTER TABLE ' || quote_ident(r.table_name) || ' DROP COLUMN organization_id';
        ELSE
            EXECUTE 'ALTER TABLE ' || quote_ident(r.table_name) || ' RENAME COLUMN organization_id TO tenant_id';
        END IF;
    END LOOP;
END $$;

-- Standardize RLS policies to use tenant_id
DO $$
DECLARE
    t_name text;
BEGIN
    FOR t_name IN (
        SELECT DISTINCT table_name
        FROM information_schema.columns
        WHERE column_name = 'tenant_id'
        AND table_schema = 'public')
    LOOP
        -- Drop old policies that might use organization_id
        EXECUTE 'DROP POLICY IF EXISTS tenant_isolation_' || t_name || ' ON ' || t_name;
        EXECUTE 'DROP POLICY IF EXISTS tenant_isolation_' || t_name || '_strict ON ' || t_name;

        -- Create new standardized policy
        -- Ensure RLS is enabled
        EXECUTE 'ALTER TABLE ' || quote_ident(t_name) || ' ENABLE ROW LEVEL SECURITY';

        EXECUTE 'CREATE POLICY tenant_isolation_' || t_name || '_strict ON ' || quote_ident(t_name) ||
                ' FOR ALL USING (tenant_id = current_setting(''app.current_tenant'', true))';
    END LOOP;
END $$;
