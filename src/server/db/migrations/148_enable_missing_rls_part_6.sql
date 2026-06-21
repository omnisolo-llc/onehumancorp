-- +goose Up
-- Migration to enable missing RLS for tables containing tenant_id

DO $$
DECLARE
    t_name text;
    tables_to_update text[] := ARRAY[
        'customers', 'products', 'services', 'orders', 'order_line_items', 'bookings',
        'ai_memories', 'interactions', 'agent_actions', 'customer_timeline',
        'vendors', 'raw_materials', 'bom_items', 'purchase_orders', 'po_line_items', 'depletion_logs'
    ];
BEGIN
    FOREACH t_name IN ARRAY tables_to_update
    LOOP
        -- Check if the table exists
        IF EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_schema = current_schema() AND table_name = t_name
        ) THEN
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
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
END $$;

-- +goose Down
DO $$
DECLARE
    t_name text;
    pol_name text;
    tables_to_update text[] := ARRAY[
        'customers', 'products', 'services', 'orders', 'order_line_items', 'bookings',
        'ai_memories', 'interactions', 'agent_actions', 'customer_timeline',
        'vendors', 'raw_materials', 'bom_items', 'purchase_orders', 'po_line_items', 'depletion_logs'
    ];
BEGIN
    FOREACH t_name IN ARRAY tables_to_update
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
        END IF;
    END LOOP;
END $$;
