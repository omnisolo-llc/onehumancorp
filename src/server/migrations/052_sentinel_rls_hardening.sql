-- 052_sentinel_rls_hardening.sql
-- Sentinel: Hardening multi-tenant isolation by adding missing organization_id columns and RLS policies.

-- 1. Add organization_id to tables that were missing it.
ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE swarm_dream_epochs ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE swarm_long_term_memory ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE swarm_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE swarm_ultra_plans ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE memory_conflicts ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';

-- 2. Enable RLS on these tables.
ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_dream_epochs ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_long_term_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_ultra_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE memory_conflicts ENABLE ROW LEVEL SECURITY;
ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;

-- 3. Create isolation policies using current_setting('app.current_tenant', true).
-- Standard policy: (organization_id = tenant OR tenant = 'system' OR tenant = '')

CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_dream_epochs ON swarm_dream_epochs USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_long_term_memory ON swarm_long_term_memory USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_task_dependencies ON swarm_task_dependencies USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_ultra_plans ON swarm_ultra_plans USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_memory_conflicts ON memory_conflicts USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

ALTER TABLE local_cloud_sync_log ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_local_cloud_sync_log ON local_cloud_sync_log USING (
    memory_id IN (SELECT "key" FROM swarm_memory WHERE organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '')
);

-- state_machine_transitions relies on task_id which refers to shared_tasks_decomposition.
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (
    task_id IN (SELECT id FROM shared_tasks_decomposition WHERE organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '')
);

-- 4. Harden hash-based caches if PII could be present.
-- Even if they are just caches, we should isolate them to prevent side-channel leaks between tenants.
ALTER TABLE llm_completion_cache ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE llm_reason_cache ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE embedding_cache ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';

ALTER TABLE llm_completion_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE llm_reason_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_llm_completion_cache ON llm_completion_cache USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_llm_reason_cache ON llm_reason_cache USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_embedding_cache ON embedding_cache USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
