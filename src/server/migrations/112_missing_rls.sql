-- +goose Up
-- Migration 112: Enforce Missing Row Level Security Policies

DO $$
DECLARE
    t_name text;
    pol_name text;
    col_type text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'agent_action_requests',
            'agent_actions',
            'agent_session_data',
            'ai_memories',
            'bom_items',
            'customer_timeline',
            'depletion_logs',
            'interactions',
            'inventory_levels',
            'order_line_items',
            'po_line_items',
            'pos_offline_transactions',
            'purchase_orders',
            'raw_materials',
            'services',
            'swarm_tasks',
            'swarm_truth_embeddings',
            'terminal_sessions',
            'vendors'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name=t_name AND column_name='tenant_id') THEN
                SELECT data_type INTO col_type FROM information_schema.columns WHERE table_name=t_name AND column_name='tenant_id';
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
                pol_name := format('tenant_isolation_%s', t_name);
                IF NOT EXISTS (
                    SELECT 1
                    FROM pg_policies
                    WHERE schemaname = current_schema()
                        AND tablename = t_name
                        AND policyname = pol_name
                ) THEN
                    IF col_type = 'uuid' THEN
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (tenant_id = current_setting(''app.current_tenant'', true)::uuid) WITH CHECK (tenant_id = current_setting(''app.current_tenant'', true)::uuid)',
                            pol_name,
                            t_name
                        );
                    ELSE
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (tenant_id = current_setting(''app.current_tenant'', true)::%s) WITH CHECK (tenant_id = current_setting(''app.current_tenant'', true)::%s)',
                            pol_name,
                            t_name,
                            col_type,
                            col_type
                        );
                    END IF;
                END IF;
            ELSIF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name=t_name AND column_name='organization_id') THEN
                SELECT data_type INTO col_type FROM information_schema.columns WHERE table_name=t_name AND column_name='organization_id';
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
                pol_name := format('tenant_isolation_%s', t_name);
                IF NOT EXISTS (
                    SELECT 1
                    FROM pg_policies
                    WHERE schemaname = current_schema()
                        AND tablename = t_name
                        AND policyname = pol_name
                ) THEN
                    IF col_type = 'uuid' THEN
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (organization_id = current_setting(''app.current_tenant'', true)::uuid) WITH CHECK (organization_id = current_setting(''app.current_tenant'', true)::uuid)',
                            pol_name,
                            t_name
                        );
                    ELSE
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (organization_id = current_setting(''app.current_tenant'', true)::%s) WITH CHECK (organization_id = current_setting(''app.current_tenant'', true)::%s)',
                            pol_name,
                            t_name,
                            col_type,
                            col_type
                        );
                    END IF;
                END IF;
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
            'agent_action_requests',
            'agent_actions',
            'agent_session_data',
            'ai_memories',
            'bom_items',
            'customer_timeline',
            'depletion_logs',
            'interactions',
            'inventory_levels',
            'order_line_items',
            'po_line_items',
            'pos_offline_transactions',
            'purchase_orders',
            'raw_materials',
            'services',
            'swarm_tasks',
            'swarm_truth_embeddings',
            'terminal_sessions',
            'vendors'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
            EXECUTE format('ALTER TABLE %I DISABLE ROW LEVEL SECURITY', t_name);
        END IF;
    END LOOP;
END
$$;
