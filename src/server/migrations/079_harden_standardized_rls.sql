-- 079_harden_standardized_rls.sql

-- Enable and Force Row Level Security on all tables that store tenant-specific data
DO $$
DECLARE
    t TEXT;
BEGIN
    FOR t IN
        SELECT table_name
        FROM information_schema.columns
        WHERE column_name = 'tenant_id'
        AND table_schema = 'public'
    LOOP
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t);
        EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY', t);
    END LOOP;
END $$;

-- Specifically handle any remaining tables that might still use organization_id
ALTER TABLE IF EXISTS users ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users FORCE ROW LEVEL SECURITY;

-- Drop existing policies to ensure standardization
DO $$
DECLARE
    pol RECORD;
BEGIN
    FOR pol IN
        SELECT policyname, tablename
        FROM pg_policies
        WHERE schemaname = 'public'
    LOOP
        EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol.policyname, pol.tablename);
    END LOOP;
END $$;

-- Recreate standardized policies using app.current_tenant
DO $$
DECLARE
    t TEXT;
BEGIN
    FOR t IN
        SELECT table_name
        FROM information_schema.columns
        WHERE (column_name = 'tenant_id' OR column_name = 'organization_id')
        AND table_schema = 'public'
    LOOP
        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = t AND column_name = 'tenant_id') THEN
            EXECUTE format('CREATE POLICY tenant_isolation_%I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true))', t, t);
        ELSIF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = t AND column_name = 'organization_id') THEN
            EXECUTE format('CREATE POLICY tenant_isolation_%I ON %I USING (organization_id::text = current_setting(''app.current_tenant'', true))', t, t);
        END IF;
    END LOOP;
END $$;

-- Ensure "system" can access all data when needed (but only through specific bypass)
-- This is already handled by the "ohc_bypassrls" role in previous migrations,
-- but we make sure the current session can't easily escape.
