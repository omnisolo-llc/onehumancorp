-- Enable RLS and setup tenant isolation policies for missing tables in migrations

DO $$
DECLARE
    t_name text;
BEGIN
    FOR t_name IN SELECT unnest(ARRAY[
        'services',
        'order_line_items',
        'ai_memories',
        'interactions',
        'agent_actions',
        'customer_timeline',
        'vendors',
        'raw_materials',
        'bom_items',
        'purchase_orders',
        'po_line_items',
        'depletion_logs'
    ])
    LOOP
        -- Check if table exists
        IF EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = t_name) THEN
            -- Enable RLS
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

            -- Create policy if it doesn't exist
            IF NOT EXISTS (
                SELECT 1 FROM pg_policies
                WHERE schemaname = 'public'
                AND tablename = t_name
                AND policyname = 'tenant_isolation_' || t_name
            ) THEN
                EXECUTE format(
                    'CREATE POLICY tenant_isolation_%I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    t_name, t_name
                );
            END IF;
        END IF;
    END LOOP;
END $$;
