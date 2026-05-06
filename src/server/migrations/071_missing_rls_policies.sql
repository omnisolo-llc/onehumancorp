ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE memory_conflicts ENABLE ROW LEVEL SECURITY;
ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE llm_completion_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE llm_reason_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE local_cloud_sync_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_dream_epochs ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_long_term_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_ultra_plans ENABLE ROW LEVEL SECURITY;

ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE memory_conflicts ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_memory_conflicts ON memory_conflicts USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE embedding_cache ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_embedding_cache ON embedding_cache USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE llm_completion_cache ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_llm_completion_cache ON llm_completion_cache USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE llm_reason_cache ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_llm_reason_cache ON llm_reason_cache USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE local_cloud_sync_log ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_local_cloud_sync_log ON local_cloud_sync_log USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_dream_epochs ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_swarm_dream_epochs ON swarm_dream_epochs USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_long_term_memory ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_swarm_long_term_memory ON swarm_long_term_memory USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_swarm_task_dependencies ON swarm_task_dependencies USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_ultra_plans ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_swarm_ultra_plans ON swarm_ultra_plans USING (organization_id::text = current_setting('app.current_tenant', true));
