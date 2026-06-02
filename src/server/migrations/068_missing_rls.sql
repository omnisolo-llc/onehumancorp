-- Migration 068: Add missing RLS policies for security hardening

-- Helper function to check if a column exists
CREATE OR REPLACE FUNCTION _ohc_col_exists(t_name text, c_name text) RETURNS boolean AS $$
DECLARE
    col_count integer;
BEGIN
    SELECT count(*) INTO col_count
    FROM information_schema.columns
    WHERE table_name = t_name AND column_name = c_name AND table_schema = current_schema();
    RETURN col_count > 0;
END;
$$ LANGUAGE plpgsql;

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'ai_memories',
            'bom_items',
            'customer360',
            'depletion_logs',
            'loyalty_ledger',
            'order_line_items',
            'po_line_items',
            'purchase_orders',
            'raw_materials',
            'services',
            'vendors',
            'agent_actions',
            'agent_memory_embeddings',
            'bus_checkpoints',
            'bus_locks',
            'bus_messages',
            'customer_timeline',
            'inbox_messages',
            'interactions',
            'local_queue_jobs',
            'sync_queue'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            IF NOT EXISTS (
                SELECT 1
                FROM pg_policies
                WHERE schemaname = current_schema()
                    AND tablename = t_name
                    AND policyname = pol_name
            ) THEN
                IF _ohc_col_exists(t_name, 'tenant_id') THEN
                    EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                ELSIF _ohc_col_exists(t_name, 'organization_id') THEN
                    EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                ELSE
                    RAISE NOTICE 'Skipping RLS policy creation for %: missing tenant_id or organization_id', t_name;
                END IF;
            END IF;
        END IF;
    END LOOP;
END
$$;

DROP FUNCTION IF EXISTS _ohc_col_exists;

-- For global fx rates
ALTER TABLE IF EXISTS ohc_fx_rates ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF to_regclass('ohc_fx_rates') IS NOT NULL THEN
        DROP POLICY IF EXISTS global_read_ohc_fx_rates ON ohc_fx_rates;
        CREATE POLICY global_read_ohc_fx_rates ON ohc_fx_rates FOR SELECT USING (true);
    END IF;
END
$$;
