-- Migration: 003_rls_hardening.sql
-- Enforce Row Level Security (RLS) on all remaining tenant-scoped tables.

-- 1. Add missing tenant_id columns to tables that lacked them
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';

-- 2. Enable RLS on all identified tables
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
ALTER TABLE competitor_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_violations ENABLE ROW LEVEL SECURITY;
ALTER TABLE hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
ALTER TABLE department_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
ALTER TABLE pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;

-- 3. Create RLS Policies using app.current_tenant
-- Each policy also checks for ohc_bypassrls role to allow system operations.

DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
    CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_agent_status ON agent_status;
    CREATE POLICY tenant_isolation_agent_status ON agent_status USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_shared_tasks_v4 ON shared_tasks_v4;
    CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
    CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_agent_approvals ON agent_approvals;
    CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
    CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
    CREATE POLICY tenant_isolation_referrals ON referrals USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_competitor_metrics ON competitor_metrics;
    CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_agent_violations ON agent_violations;
    CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue;
    CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
    CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (current_user = 'ohc_bypassrls' OR organization_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_department_tasks ON department_tasks;
    CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
    CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
    CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_pages ON pages;
    CREATE POLICY tenant_isolation_pages ON pages USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_memories ON memories;
    CREATE POLICY tenant_isolation_memories ON memories USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
    CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
    CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
    CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
    CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_agent_session_data ON agent_session_data;
    CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
    CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));

    DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
    CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (current_user = 'ohc_bypassrls' OR tenant_id::text = current_setting('app.current_tenant', true));
END $$;

-- 4. Ensure Indexes exist for performance of RLS filtering
CREATE INDEX IF NOT EXISTS idx_agent_missions_tenant_id ON agent_missions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_status_tenant_id ON agent_status(tenant_id);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_v4_tenant_id ON shared_tasks_v4(tenant_id);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_tenant_id ON shared_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_approvals_tenant_id ON agent_approvals(tenant_id);
CREATE INDEX IF NOT EXISTS idx_onboarding_state_tenant_id ON onboarding_state(tenant_id);
CREATE INDEX IF NOT EXISTS idx_referrals_tenant_id ON referrals(tenant_id);
CREATE INDEX IF NOT EXISTS idx_competitor_metrics_tenant_id ON competitor_metrics(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_violations_tenant_id ON agent_violations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_hybrid_fs_sync_queue_tenant_id ON hybrid_fs_sync_queue(tenant_id);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_decomposition_org_id ON shared_tasks_decomposition(organization_id);
CREATE INDEX IF NOT EXISTS idx_department_tasks_tenant_id ON department_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_autodream_memories_tenant_id ON autodream_memories(tenant_id);
CREATE INDEX IF NOT EXISTS idx_state_machine_transitions_tenant_id ON state_machine_transitions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_pages_tenant_id ON pages(tenant_id);
CREATE INDEX IF NOT EXISTS idx_memories_tenant_id ON memories(tenant_id);
CREATE INDEX IF NOT EXISTS idx_consolidated_memory_tenant_id ON consolidated_memory(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_inbox_tenant_id ON agent_inbox(tenant_id);
CREATE INDEX IF NOT EXISTS idx_meeting_rooms_tenant_id ON meeting_rooms(tenant_id);
CREATE INDEX IF NOT EXISTS idx_meeting_transcripts_tenant_id ON meeting_transcripts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_session_data_tenant_id ON agent_session_data(tenant_id);
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_tenant_id ON swarm_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_swarm_truth_embeddings_tenant_id ON swarm_truth_embeddings(tenant_id);
