-- +goose Up
-- Enable missing Row Level Security for tables
-- Following the existing pattern: ALTER TABLE %I ENABLE ROW LEVEL SECURITY; and CREATE POLICY ...

ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'agent_actions',
            'ai_memories',
            'bom_items',
            'customer_timeline',
            'depletion_logs',
            'interactions',
            'order_line_items',
            'po_line_items',
            'raw_materials'
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

-- +goose Down
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'agent_actions',
            'ai_memories',
            'bom_items',
            'customer_timeline',
            'depletion_logs',
            'interactions',
            'order_line_items',
            'po_line_items',
            'raw_materials'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
        END IF;
    END LOOP;
END
$$;

ALTER TABLE IF EXISTS agent_actions DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ai_memories DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bom_items DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer_timeline DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS depletion_logs DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS interactions DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS order_line_items DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS po_line_items DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS raw_materials DISABLE ROW LEVEL SECURITY;
