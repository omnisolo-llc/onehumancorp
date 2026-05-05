-- 067_harden_remaining_rls_policies_2.sql

ALTER TABLE department_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_department_tasks_t ON department_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE shared_tasks_v4 ADD COLUMN IF NOT EXISTS tenant_id TEXT;
DO \$\$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'shared_tasks_v4' AND column_name = 'organization_id') THEN
        UPDATE shared_tasks_v4 SET tenant_id = organization_id WHERE tenant_id IS NULL;
    END IF;
END \$\$;
ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_v4_t ON shared_tasks_v4 USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_sub_agent_jobs_t ON sub_agent_jobs USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_session_data_t ON agent_session_data USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE swarm_dream_epochs ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_dream_epochs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_dream_epochs_t ON swarm_dream_epochs USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE memory_conflicts ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE memory_conflicts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_memory_conflicts_t ON memory_conflicts USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE swarm_task_dependencies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_task_dependencies_t ON swarm_task_dependencies USING (task_id IN (SELECT id FROM swarm_tasks WHERE tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system'));

ALTER TABLE swarm_long_term_memory ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_long_term_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_long_term_memory_t ON swarm_long_term_memory USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE llm_completion_cache ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_llm_completion_cache_t ON llm_completion_cache USING (true); -- Cache tables can be shared or we could add tenant_id

ALTER TABLE llm_reason_cache ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_llm_reason_cache_t ON llm_reason_cache USING (true); -- Cache tables can be shared

ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_embedding_cache_t ON embedding_cache USING (true); -- Cache tables can be shared

ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_truth_embeddings_t ON swarm_truth_embeddings USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE local_cloud_sync_log ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE local_cloud_sync_log ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_local_cloud_sync_log_t ON local_cloud_sync_log USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE swarm_ultra_plans ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_ultra_plans ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_ultra_plans_t ON swarm_ultra_plans USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_roles_t ON roles USING (true); -- Roles are system wide.

ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_revoked_tokens_t ON revoked_tokens USING (true); -- Tokens are system wide.
