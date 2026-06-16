-- Migration to ensure all tables have explicit RLS policies

DO $$
DECLARE
    t_name text;
    pol_name text;
    has_tenant boolean;
    has_org boolean;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'agent_actions'
            ,'agent_session_data'
            ,'ai_memories'
            ,'bom_items'
            ,'builder_brand_toolboxes'
            ,'customer360'
            ,'customer_timeline'
            ,'delivery_tasks'
            ,'depletion_logs'
            ,'embedding_cache'
            ,'interactions'
            ,'loyalty_ledger'
            ,'ohc_fx_rates'
            ,'ohc_i18n_strings'
            ,'ohc_multi_currency_ledger'
            ,'ohc_translation_preferences'
            ,'order_line_items'
            ,'po_line_items'
            ,'raw_materials'
            ,'services'
            ,'sub_agent_queue'
            ,'swarm_tasks'
            ,'swarm_truth_embeddings'
            ,'task_dependencies'
            ,'telemetry_buffer'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN

            SELECT EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = t_name AND column_name = 'tenant_id'
            ) INTO has_tenant;

            SELECT EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = t_name AND column_name = 'organization_id'
            ) INTO has_org;

            -- Only enable RLS if the table is actually tenant-scoped
            IF has_tenant OR has_org THEN
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

                IF has_tenant THEN
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
                ELSIF has_org THEN
                    pol_name := format('tenant_isolation_%s', t_name);
                    IF NOT EXISTS (
                        SELECT 1
                        FROM pg_policies
                        WHERE schemaname = current_schema()
                            AND tablename = t_name
                            AND policyname = pol_name
                    ) THEN
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true))',
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
