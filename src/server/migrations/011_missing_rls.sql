-- Add missing RLS policies to tables that were lacking them.
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true));
