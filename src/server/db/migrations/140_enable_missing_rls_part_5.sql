-- Enable RLS for tables containing tenant_id that are missing it


-- Add tenant_isolation policies for all tables missing it
DO $$
DECLARE
    t_name text;
    tables_to_update text[] := ARRAY[

    ];
BEGIN
    FOREACH t_name IN ARRAY tables_to_update
    LOOP
        -- Check if the table exists
        IF EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_schema = current_schema() AND table_name = t_name
        ) THEN
            -- Check if policy exists
            IF NOT EXISTS (
                SELECT 1 FROM pg_policies
                WHERE schemaname = current_schema()
                  AND tablename = t_name
                  AND policyname = 'tenant_isolation_' || t_name
            ) THEN
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY; CREATE POLICY tenant_isolation_%I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))', t_name, t_name);
            END IF;
        END IF;
    END LOOP;
END $$;

-- Add missing tenant_isolation policies

DO $$
DECLARE
    t_name text;
    tables_to_update text[] := ARRAY[
        'agent_actions', 'ai_memories', 'bom_items', 'customer_timeline',
        'depletion_logs', 'interactions', 'order_line_items', 'po_line_items',
        'raw_materials'
    ];
BEGIN
    FOREACH t_name IN ARRAY tables_to_update
    LOOP
        -- Check if the table exists
        IF EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_schema = current_schema() AND table_name = t_name
        ) THEN
            -- Check if policy exists
            IF NOT EXISTS (
                SELECT 1 FROM pg_policies
                WHERE schemaname = current_schema()
                  AND tablename = t_name
                  AND policyname = 'tenant_isolation_' || t_name
            ) THEN
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY; CREATE POLICY tenant_isolation_%I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))', t_name, t_name);
            END IF;
        END IF;
    END LOOP;
END $$;
