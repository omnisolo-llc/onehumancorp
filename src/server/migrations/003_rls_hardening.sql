-- Migration: 003_rls_hardening.sql
-- Description: Enable Row Level Security (RLS) on all remaining tenant-scoped tables
-- to ensure strict multi-tenant isolation in Cloud mode.

-- 1. agent_missions
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 2. agent_status
ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_status ON agent_status;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 3. shared_tasks_v4
ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_v4 ON shared_tasks_v4;
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 4. shared_tasks
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 5. agent_approvals
ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_approvals ON agent_approvals;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 6. onboarding_state
ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 7. referrals
ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 8. competitor_metrics
ALTER TABLE competitor_metrics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_competitor_metrics ON competitor_metrics;
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 9. agent_violations
ALTER TABLE agent_violations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_violations ON agent_violations;
CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 10. hybrid_fs_sync_queue
ALTER TABLE hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue;
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 11. shared_tasks_decomposition (uses organization_id)
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true));

-- 12. department_tasks
ALTER TABLE department_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_department_tasks ON department_tasks;
CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 13. autodream_memories
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 14. state_machine_transitions
ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 15. pages
ALTER TABLE pages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pages ON pages;
CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 16. memories
ALTER TABLE memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_memories ON memories;
CREATE POLICY tenant_isolation_memories ON memories USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 17. consolidated_memory
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 18. agent_inbox
ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 19. meeting_rooms
ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 20. meeting_transcripts
ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (tenant_id::text = current_setting('app.current_tenant', true));
