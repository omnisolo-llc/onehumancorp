-- +goose Up
-- Migration 147: Enforce RLS on assistant tables
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'assistant_workspaces',
            'assistant_tasks',
            'assistant_messages',
            'assistant_artifacts',
            'assistant_file_changes',
            'assistant_memory_records',
            'assistant_skills',
            'assistant_connectors'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY', t_name);
            EXECUTE format('ALTER TABLE IF EXISTS %I FORCE ROW LEVEL SECURITY', t_name);
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
