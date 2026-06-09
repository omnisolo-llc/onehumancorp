-- +goose Up
-- Migration 111: Explicitly Add Row Level Security to Older Tables to resolve audit warnings

DO $$
DECLARE
    t_name text;
    pol_name text;
    col_type text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'customers',
            'products',
            'services',
            'orders',
            'order_line_items',
            'bookings',
            'ai_memories',
            'interactions',
            'agent_actions',
            'customer_timeline',
            'business_milestones',
            'vendors',
            'raw_materials',
            'bom_items',
            'purchase_orders',
            'po_line_items',
            'depletion_logs',
            'shared_tasks_decomposition',
            'shared_task_dependencies',
            'team_invites'
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
                            'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                            pol_name,
                            t_name
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
                            'CREATE POLICY %I ON %I USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true))',
                            pol_name,
                            t_name
                        );
                    END IF;
                END IF;
            ELSIF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name=t_name AND column_name='team_id') THEN
                SELECT data_type INTO col_type FROM information_schema.columns WHERE table_name=t_name AND column_name='team_id';
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
                            'CREATE POLICY %I ON %I USING (team_id = current_setting(''app.current_tenant'', true)::uuid) WITH CHECK (team_id = current_setting(''app.current_tenant'', true)::uuid)',
                            pol_name,
                            t_name
                        );
                    ELSE
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (team_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (team_id::text = current_setting(''app.current_tenant'', true))',
                            pol_name,
                            t_name
                        );
                    END IF;
                END IF;
            END IF;
        END IF;
    END LOOP;
END
$$;

-- +goose Down
-- Reverting this globally might break environments where this was enforced programmatically. Therefore this is a no-op down.
