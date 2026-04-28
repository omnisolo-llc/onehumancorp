-- 044_tenant_rls.sql
-- Enforce Row Level Security on all tables that support multi-tenant isolation.

ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE capability_plugins ENABLE ROW LEVEL SECURITY;
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
ALTER TABLE scheduled_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_memory_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
ALTER TABLE usage_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- Creating policies
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_agents ON agents USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_capability_plugins ON capability_plugins USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_scheduled_tasks ON scheduled_tasks USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_swarm_memory ON swarm_memory USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_tasks ON tasks USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_usage_events ON usage_events USING (organization_id = current_setting('app.current_tenant'));
CREATE POLICY tenant_isolation_users ON users USING (organization_id = current_setting('app.current_tenant'));
