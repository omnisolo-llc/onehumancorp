-- Up
DO $$
BEGIN
    IF to_regclass('public.meeting_rooms') IS NOT NULL THEN
        ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS "Tenant isolation for meeting_rooms" ON meeting_rooms;
        CREATE POLICY "Tenant isolation for meeting_rooms" ON meeting_rooms
            USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;

    IF to_regclass('public.meeting_transcripts') IS NOT NULL THEN
        ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS "Tenant isolation for meeting_transcripts" ON meeting_transcripts;
        CREATE POLICY "Tenant isolation for meeting_transcripts" ON meeting_transcripts
            USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;

    IF to_regclass('public.business_milestones') IS NOT NULL THEN
        ALTER TABLE business_milestones ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS "Tenant isolation for business_milestones" ON business_milestones;
        CREATE POLICY "Tenant isolation for business_milestones" ON business_milestones
            USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;

    IF to_regclass('public.shared_tasks_decomposition') IS NOT NULL THEN
        ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS "Tenant isolation for shared_tasks_decomposition" ON shared_tasks_decomposition;
        CREATE POLICY "Tenant isolation for shared_tasks_decomposition" ON shared_tasks_decomposition
            USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;

    IF to_regclass('public.interactions') IS NOT NULL THEN
        ALTER TABLE interactions ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS "Tenant isolation for interactions" ON interactions;
        CREATE POLICY "Tenant isolation for interactions" ON interactions
            USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;

    IF to_regclass('public.agent_actions') IS NOT NULL THEN
        ALTER TABLE agent_actions ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS "Tenant isolation for agent_actions" ON agent_actions;
        CREATE POLICY "Tenant isolation for agent_actions" ON agent_actions
            USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;

    IF to_regclass('public.customer_timeline') IS NOT NULL THEN
        ALTER TABLE customer_timeline ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS "Tenant isolation for customer_timeline" ON customer_timeline;
        CREATE POLICY "Tenant isolation for customer_timeline" ON customer_timeline
            USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;
END $$;

-- Down
DO $$
BEGIN
    IF to_regclass('public.meeting_rooms') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for meeting_rooms" ON meeting_rooms;
    END IF;
    IF to_regclass('public.meeting_transcripts') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for meeting_transcripts" ON meeting_transcripts;
    END IF;
    IF to_regclass('public.business_milestones') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for business_milestones" ON business_milestones;
    END IF;
    IF to_regclass('public.shared_tasks_decomposition') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for shared_tasks_decomposition" ON shared_tasks_decomposition;
    END IF;
    IF to_regclass('public.interactions') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for interactions" ON interactions;
    END IF;
    IF to_regclass('public.agent_actions') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for agent_actions" ON agent_actions;
    END IF;
    IF to_regclass('public.customer_timeline') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for customer_timeline" ON customer_timeline;
    END IF;
END $$;
