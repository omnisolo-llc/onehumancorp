-- 049_missing_rls.sql
-- Enable Row Level Security and corresponding policies on multi-tenant tables.

-- In PostgreSQL, enabling RLS without policies results in default-deny.
-- The previous migration 046_rls_enable.sql enabled RLS but did not add policies.
-- We must add policies using the current_setting('app.current_tenant', true) convention.

-- Tables that use organization_id:
CREATE POLICY tenant_isolation_tasks ON tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_memory ON swarm_memory USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_capability_plugins ON capability_plugins USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_usage_events ON usage_events USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_users ON users USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_scheduled_tasks ON scheduled_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_products ON products USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_agents ON agents USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- Tables that use tenant_id:
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
CREATE POLICY tenant_isolation_local_mcp_rag_tasks ON local_mcp_rag_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- Add missing RLS to tables that should have it but were missed:
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;

-- Note: task_dependencies and sub_agent_queue don't have organization_id directly in some early schemas, but sub_agent_queue has it in the code (`pub organization_id: String,`).
-- Let's make sure:
ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- task_dependencies doesn't have an org id, it relies on task_id.
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (
    task_id IN (SELECT id FROM shared_tasks WHERE organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '')
);
