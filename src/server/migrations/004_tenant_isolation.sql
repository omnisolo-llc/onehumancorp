-- Migration 004: Enforce Hybrid Multi-tenant RLS Policies

-- For tables that have `tenant_id`
ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_v4 ON shared_tasks_v4;
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_approvals ON agent_approvals;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE competitor_metrics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_competitor_metrics ON competitor_metrics;
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_violations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_violations ON agent_violations;
CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue;
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE department_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_department_tasks ON department_tasks;
CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE pages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pages ON pages;
CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_memories ON memories;
CREATE POLICY tenant_isolation_memories ON memories USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Handle table with 'organization_id' instead of 'tenant_id'
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true));

-- Handle tables without direct 'tenant_id' or where the rule asks to use a subquery check
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_session_data ON agent_session_data;
CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (agent_id IN (SELECT id FROM agents WHERE tenant_id::text = current_setting('app.current_tenant', true)));

ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (memory_id IN (SELECT id FROM agent_memories WHERE tenant_id::text = current_setting('app.current_tenant', true)));

ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (mission_id IN (SELECT id FROM agent_missions WHERE tenant_id::text = current_setting('app.current_tenant', true)));

-- Subquery for meeting_transcripts as requested by prompt rule despite having a tenant_id column
ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (meeting_id IN (SELECT id FROM meeting_rooms WHERE tenant_id::text = current_setting('app.current_tenant', true)));
