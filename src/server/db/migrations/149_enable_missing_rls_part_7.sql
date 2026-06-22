-- +goose Up
-- Migration to systematically add missing RLS policies to any tenant-related tables
DO $$
DECLARE
    t_name text;
    tables_to_update text[] := ARRAY[
        'embedding_cache', 'agent_actions', 'ai_memories', 'bom_items',
        'customer_timeline', 'depletion_logs', 'interactions',
        'order_line_items', 'po_line_items', 'raw_materials'
    ];
BEGIN
    FOREACH t_name IN ARRAY tables_to_update
    LOOP
        -- Check if the table exists
        IF EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_schema = current_schema() AND table_name = t_name
        ) THEN
            -- Make sure the table has RLS enabled
            EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY', t_name);

            -- Check if policy exists
            IF NOT EXISTS (
                SELECT 1 FROM pg_policies
                WHERE schemaname = current_schema()
                  AND tablename = t_name
                  AND policyname = 'tenant_isolation_' || t_name
            ) THEN
                EXECUTE format('CREATE POLICY tenant_isolation_%I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))', t_name, t_name);
            END IF;
        END IF;
    END LOOP;

    -- Dynamic fallback loop: check ALL tables that have tenant_id column but no RLS policy
    FOR t_name IN
        SELECT table_name
        FROM information_schema.columns
        WHERE column_name IN ('tenant_id', 'organization_id') AND table_schema = 'public'
    LOOP
        -- Exclude tables that use a different isolation mechanism or aren't meant for row level security
        IF t_name NOT IN ('pg_stat_statements', 'spatial_ref_sys') THEN
            EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY', t_name);

            IF NOT EXISTS (
                SELECT 1 FROM pg_policies
                WHERE schemaname = 'public'
                  AND tablename = t_name
            ) THEN
                -- Find the exact column name (tenant_id vs organization_id)
                DECLARE
                    col_name text;
                BEGIN
                    SELECT column_name INTO col_name
                    FROM information_schema.columns
                    WHERE table_name = t_name AND column_name IN ('tenant_id', 'organization_id')
                    LIMIT 1;

                    EXECUTE format('CREATE POLICY tenant_isolation_dynamic_%I ON %I USING (%I::text = current_setting(''app.current_tenant'', true)) WITH CHECK (%I::text = current_setting(''app.current_tenant'', true))', t_name, t_name, col_name, col_name);
                EXCEPTION
                    WHEN OTHERS THEN
                        -- Ignore errors for tables like views where policy creation fails
                        NULL;
                END;
            END IF;
        END IF;
    END LOOP;
END $$;

-- +goose Down
-- Reverting dynamic RLS enforcement is complex and potentially destructive, so we keep this minimal or omit
