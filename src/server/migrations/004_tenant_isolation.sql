-- Migration 004: Enforce Hybrid Multi-tenant RLS Policies

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'shared_tasks_v4',
            'shared_tasks',
            'agent_approvals',
            'onboarding_state',
            'referrals',
            'competitor_metrics',
            'agent_violations',
            'hybrid_fs_sync_queue',
            'department_tasks',
            'autodream_memories',
            'state_machine_transitions',
            'pages',
            'memories',
            'consolidated_memory',
            'agent_inbox',
            'meeting_rooms'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

            pol_name := format('tenant_isolation_%s', t_name);
            IF NOT EXISTS (
                SELECT 1
                FROM pg_policies
                WHERE schemaname = current_schema()
                    AND tablename = t_name
                    AND policyname = pol_name
            ) THEN
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        END IF;
    END LOOP;
END
$$;

DO $$
BEGIN
    IF to_regclass('shared_tasks_decomposition') IS NOT NULL THEN
        ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'shared_tasks_decomposition'
                AND policyname = 'tenant_isolation_shared_tasks_decomposition'
        ) THEN
            CREATE POLICY tenant_isolation_shared_tasks_decomposition
                ON shared_tasks_decomposition
                USING (organization_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

DO $$
BEGIN
    IF to_regclass('agent_session_data') IS NOT NULL THEN
        ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'agent_session_data'
                AND policyname = 'tenant_isolation_agent_session_data'
        ) THEN
            CREATE POLICY tenant_isolation_agent_session_data
                ON agent_session_data
                USING (agent_id IN (SELECT id FROM agents WHERE tenant_id::text = current_setting('app.current_tenant', true)));
        END IF;
    END IF;

    IF to_regclass('swarm_truth_embeddings') IS NOT NULL THEN
        ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'swarm_truth_embeddings'
                AND policyname = 'tenant_isolation_swarm_truth_embeddings'
        ) THEN
            CREATE POLICY tenant_isolation_swarm_truth_embeddings
                ON swarm_truth_embeddings
                USING (memory_id IN (SELECT id FROM agent_memories WHERE tenant_id::text = current_setting('app.current_tenant', true)));
        END IF;
    END IF;

    IF to_regclass('swarm_tasks') IS NOT NULL THEN
        ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'swarm_tasks'
                AND policyname = 'tenant_isolation_swarm_tasks'
        ) THEN
            CREATE POLICY tenant_isolation_swarm_tasks
                ON swarm_tasks
                USING (mission_id IN (SELECT id FROM agent_missions WHERE tenant_id::text = current_setting('app.current_tenant', true)));
        END IF;
    END IF;

    IF to_regclass('meeting_transcripts') IS NOT NULL THEN
        ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'meeting_transcripts'
                AND policyname = 'tenant_isolation_meeting_transcripts'
        ) THEN
            CREATE POLICY tenant_isolation_meeting_transcripts
                ON meeting_transcripts
                USING (meeting_id IN (SELECT id FROM meeting_rooms WHERE tenant_id::text = current_setting('app.current_tenant', true)));
        END IF;
    END IF;
END
$$;
