-- +goose Up
-- Migration 072: Enforce Missing Row Level Security Policies

DO $$
DECLARE
    t_name text;
    pol_name text;
    col_type text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'interactions',
            'agent_actions',
            'mcp_tools',
            'invoices',
            'invoice_line_items',
            'payment_events',
            'ledger_entries',
            'local_queue_jobs',
            'customer_timeline',
            'timecard_events',
            'staff_members',
            'agent_memory_embeddings',
            'bus_checkpoints',
            'bus_messages',
            'bus_locks',
            'sync_queue'
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
