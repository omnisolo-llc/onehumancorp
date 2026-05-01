-- 049_missing_rls.sql
-- Enable Row Level Security and corresponding policies on multi-tenant tables.

-- In PostgreSQL, enabling RLS without policies results in default-deny.
-- The previous migration 046_rls_enable.sql enabled RLS but did not add policies.
-- We must add policies using the current_setting('app.current_tenant', true) convention.

-- Tables that use tenant_id:
CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_swarm_memory ON swarm_memory USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_capability_plugins ON capability_plugins USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_usage_events ON usage_events USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_users ON users USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_scheduled_tasks ON scheduled_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_products ON products USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Tables that use tenant_id:
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_local_mcp_rag_tasks ON local_mcp_rag_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Add missing RLS to tables that should have it but were missed:
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;

-- Note: task_dependencies and sub_agent_queue don't have tenant_id directly in some early schemas, but sub_agent_queue has it in the code (`pub tenant_id: String,`).
-- Let's make sure:
ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- task_dependencies doesn't have an org id, it relies on task_id.
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (
    task_id IN (SELECT id FROM shared_tasks WHERE tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system')
);
