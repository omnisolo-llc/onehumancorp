-- 055_tenant_isolation_fix.sql

ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE memory_conflicts ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE memory_conflicts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_memory_conflicts ON memory_conflicts USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE swarm_long_term_memory ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE swarm_long_term_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_long_term_memory ON swarm_long_term_memory USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE revoked_tokens ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
