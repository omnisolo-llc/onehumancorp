-- +goose Up
-- Migration 133: Ensure all tenant-specific tables have ROW LEVEL SECURITY and a tenant_isolation policy

DO $$
DECLARE
    t_name text;
    c_name text;
    policy_name text;
BEGIN
    FOR t_name, c_name IN
        SELECT table_name, column_name
        FROM information_schema.columns
        WHERE table_schema = 'public' AND (column_name = 'tenant_id' OR column_name = 'organization_id')
    LOOP
        -- Enable RLS
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

        -- Create a generic tenant isolation policy if none exists
        policy_name := format('tenant_isolation_%s', t_name);

        IF NOT EXISTS (
            SELECT 1 FROM pg_policies
            WHERE schemaname = 'public'
              AND tablename = t_name
              AND policyname = policy_name
        ) THEN
            EXECUTE format('
                CREATE POLICY %I ON %I
                USING (%I::text = current_setting(''app.current_tenant'', true))
                WITH CHECK (%I::text = current_setting(''app.current_tenant'', true))
            ', policy_name, t_name, c_name, c_name);
        END IF;
    END LOOP;
END
$$;

-- +goose Down
-- Reverting this migration is not recommended as it leaves tables vulnerable to data leaks.
-- If required, you would need to drop the dynamically created policies manually.
