-- Migration: 003_rls_hardening.sql
-- Enable RLS and add missing tenant isolation columns for hybrid parity.

-- 1. Add missing tenant isolation columns
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE onboarding_state ADD COLUMN IF NOT EXISTS organization_id TEXT;
ALTER TABLE hybrid_fs_sync_queue ADD COLUMN IF NOT EXISTS organization_id TEXT;

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
-- We use robust checks for both tenant_id and organization_id to handle codebase inconsistencies.

CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id = current_setting('app.current_tenant', true) OR organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (tenant_id = current_setting('app.current_tenant', true) OR organization_id = current_setting('app.current_tenant', true));
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

-- 4. Harden existing users policy (handle possible column rename from tenant_id to organization_id in code)
DO $$
BEGIN
    IF EXISTS (SELECT FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'tenant_id') THEN
        IF NOT EXISTS (SELECT FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'organization_id') THEN
            ALTER TABLE users ADD COLUMN organization_id TEXT;
            UPDATE users SET organization_id = tenant_id;
        END IF;
    END IF;
END
$$;

DROP POLICY IF EXISTS tenant_isolation_users ON users;
CREATE POLICY tenant_isolation_users ON users USING (tenant_id = current_setting('app.current_tenant', true) OR organization_id = current_setting('app.current_tenant', true));

-- 5. Ensure ohc_bypassrls role is correctly configured
DO $$
BEGIN
    IF EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN
        EXECUTE 'ALTER ROLE ohc_bypassrls BYPASSRLS';
    END IF;
END
$$;
