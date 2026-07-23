-- +goose Up
-- Add missing RLS policies safely using PL/pgSQL
-- This catches all tables that have a tenant_id but do not have RLS enabled.

DO $$
DECLARE
    t_name text;
    policy_name text;
BEGIN
    FOR t_name IN
        SELECT c.table_name
        FROM information_schema.columns c
        JOIN information_schema.tables t ON c.table_name = t.table_name
        WHERE c.table_schema = 'public'
          AND c.column_name = 'tenant_id'
          AND t.table_type = 'BASE TABLE'
    LOOP
        -- Enable RLS (idempotent)
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY;', t_name);

        -- Shorten policy name if it would exceed 63 characters
        policy_name := 'tenant_isolation_' || t_name;
        IF length(policy_name) > 63 THEN
            policy_name := left(policy_name, 63);
        END IF;

        -- Check if policy already exists
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = 'public'
              AND tablename = t_name
              AND policyname = policy_name
        ) THEN
            EXECUTE format('CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true));', policy_name, t_name);
        END IF;
    END LOOP;
END
$$;

-- +goose Down
-- Removing policies that were added generically is risky and depends on what existed before,
-- so we generally don't remove RLS completely. We only remove the policy if we created it.

DO $$
DECLARE
    t_name text;
    policy_name text;
BEGIN
    FOR t_name IN
        SELECT table_name
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND column_name = 'tenant_id'
    LOOP
        policy_name := 'tenant_isolation_' || t_name;
        IF length(policy_name) > 63 THEN
            policy_name := left(policy_name, 63);
        END IF;

        IF EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = 'public'
              AND tablename = t_name
              AND policyname = policy_name
        ) THEN
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I;', policy_name, t_name);
        END IF;
    END LOOP;
END
$$;
