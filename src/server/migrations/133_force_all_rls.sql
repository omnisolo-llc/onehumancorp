-- +goose Up
-- Migration 133: Systematically enforce FORCE ROW LEVEL SECURITY on all tenant-specific tables

DO $$
DECLARE
    t_name text;
BEGIN
    FOR t_name IN
        SELECT tablename
        FROM pg_tables
        WHERE schemaname = 'public'
    LOOP
        -- Only target tables that have a tenant_id or organization_id column
        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name=t_name AND (column_name='tenant_id' OR column_name='organization_id')) THEN
            EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY', t_name);
        END IF;
    END LOOP;
END
$$;

-- +goose Down
-- We don't drop the policies globally here to avoid destructive rollback of previously enforced RLS
