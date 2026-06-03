-- Enable RLS on remaining missing tables
DO $$ BEGIN IF to_regclass('loyalty_ledger') IS NOT NULL THEN EXECUTE 'ALTER TABLE loyalty_ledger ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('customer360') IS NOT NULL THEN EXECUTE 'ALTER TABLE customer360 ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('agent_actions') IS NOT NULL THEN EXECUTE 'ALTER TABLE agent_actions ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('interactions') IS NOT NULL THEN EXECUTE 'ALTER TABLE interactions ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('inbox_messages') IS NOT NULL THEN EXECUTE 'ALTER TABLE inbox_messages ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('customer_timeline') IS NOT NULL THEN EXECUTE 'ALTER TABLE customer_timeline ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('agent_session_data') IS NOT NULL THEN EXECUTE 'ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('swarm_truth_embeddings') IS NOT NULL THEN EXECUTE 'ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY'; END IF; END $$;
DO $$ BEGIN IF to_regclass('swarm_tasks') IS NOT NULL THEN EXECUTE 'ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY'; END IF; END $$;

-- Define RLS Policies
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'loyalty_ledger',
            'customer360',
            'agent_actions',
            'interactions',
            'inbox_messages',
            'customer_timeline',
            'agent_session_data',
            'swarm_truth_embeddings',
            'swarm_tasks'
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
