-- Migration: 003_add_rls_policies.sql
-- Add missing RLS policies to tables created in 002_missing_tables.sql

-- For tables with tenant_id explicitly
ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE competitor_metrics ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_violations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE department_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE pages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE memories ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_memories ON memories USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (tenant_id::text = current_setting('app.current_tenant', true));

-- For tables with organization_id explicitly
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true));

-- For tables missing tenant_id but need isolation, we add tenant_id to allow isolation, fallback to null/system.
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true) OR tenant_id IS NULL);

ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id::text = current_setting('app.current_tenant', true) OR tenant_id IS NULL);

ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true) OR tenant_id IS NULL);
