-- Migration: 003_feature.sql
-- Apply RLS and BYPASSRLS securely for existing deployments

DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN
        CREATE ROLE ohc_bypassrls;
    END IF;
END
$$;

ALTER ROLE ohc_bypassrls BYPASSRLS;

-- Ensure RLS is enabled and forced for all new multi-tenant tables
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'shared_tasks_v4', 'shared_tasks', 'agent_approvals', 'onboarding_state',
            'referrals', 'competitor_metrics', 'agent_violations', 'hybrid_fs_sync_queue',
            'department_tasks', 'autodream_memories', 'state_machine_transitions',
            'pages', 'memories', 'consolidated_memory', 'agent_inbox', 'meeting_rooms', 'meeting_transcripts',
            'tenants', 'users', 'agents', 'tasks', 'products', 'orders', 'customers', 'bookings',
            'agent_memories', 'knowledge_embeddings'
        ])
    LOOP
        EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY', t_name);
        EXECUTE format('ALTER TABLE IF EXISTS %I FORCE ROW LEVEL SECURITY', t_name);

        pol_name := format('tenant_isolation_%s', t_name);

        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE policyname = pol_name AND tablename = t_name
        ) THEN
            IF t_name IN ('shared_tasks_v4', 'shared_tasks', 'agent_approvals', 'onboarding_state', 'referrals', 'competitor_metrics', 'agent_violations', 'hybrid_fs_sync_queue', 'department_tasks', 'autodream_memories', 'state_machine_transitions', 'pages', 'memories', 'consolidated_memory', 'agent_inbox', 'meeting_rooms', 'meeting_transcripts', 'tenants', 'users', 'agents', 'tasks', 'products', 'orders', 'customers', 'bookings', 'agent_memories', 'knowledge_embeddings') THEN
                EXECUTE format('CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))', pol_name, t_name);
            END IF;
        END IF;
    END LOOP;
END
$$;

DO $$
BEGIN
    EXECUTE format('ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY');
    EXECUTE format('ALTER TABLE IF EXISTS shared_tasks_decomposition FORCE ROW LEVEL SECURITY');
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_shared_tasks_decomposition' AND tablename = 'shared_tasks_decomposition') THEN
        EXECUTE format('CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true))');
    END IF;
END
$$;
