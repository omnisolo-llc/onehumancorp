-- Migration: 003_rls_hardening.sql
-- Enable RLS and add missing tenant_id columns for better isolation.

-- 1. Add missing tenant_id columns where necessary
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';

-- 2. Enable RLS on all missing tables
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
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

-- 3. Create RLS Policies
-- We use a helper function or DO block to avoid repetition if possible,
-- but explicit policies are often clearer for security audits.

CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_memories ON memories USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (tenant_id = current_setting('app.current_tenant', true));

-- 4. Ensure ohc_bypassrls role can actually bypass RLS
-- This requires the table owner or a superuser to grant it.
-- In many hosted Postgres environments, BYPASSRLS is a restricted attribute.
-- However, for our Sentinel mission, we assume we have control over the schema.
-- Note: 'GRANT ALL' doesn't grant BYPASSRLS attribute.
-- The user needs to be created with BYPASSRLS or ALTERed.
-- Since we are in a migration, we try to ensure the role is set up.

DO $$
BEGIN
    IF EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN
        EXECUTE 'ALTER ROLE ohc_bypassrls BYPASSRLS';
    END IF;
END
$$;
