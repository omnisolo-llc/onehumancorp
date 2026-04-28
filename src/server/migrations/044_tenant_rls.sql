-- 044_tenant_rls.sql
-- Enforce Row Level Security on all tables that support multi-tenant isolation.

-- First, enable RLS on all tables that have organization_id
ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE capability_plugins ENABLE ROW LEVEL SECURITY;
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE llm_completion_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE local_cloud_sync_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
ALTER TABLE scheduled_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_memory_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
ALTER TABLE usage_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- Then create policies based on organization_id
-- We use current_setting('app.current_tenant', true) so that it doesn't fail if not set,
-- and falls back to allowing superusers or bypassrls roles to do everything.
-- For a strict isolation, the setting must be matched.
CREATE POLICY tenant_isolation_policy_agent_inbox ON agent_inbox FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_agent_memories ON agent_memories FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_agent_missions ON agent_missions FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_agent_status ON agent_status FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_agents ON agents FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_autodream_memories ON autodream_memories FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_capability_plugins ON capability_plugins FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_consolidated_memory ON consolidated_memory FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_llm_completion_cache ON llm_completion_cache FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_local_cloud_sync_log ON local_cloud_sync_log FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_meeting_rooms ON meeting_rooms FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_revoked_tokens ON revoked_tokens FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_scheduled_tasks ON scheduled_tasks FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_shared_tasks ON shared_tasks FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_sub_agent_queue ON sub_agent_queue FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_swarm_memory ON swarm_memory FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_swarm_memory_embeddings ON swarm_memory_embeddings FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_tasks ON tasks FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_telemetry_buffer ON telemetry_buffer FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_usage_events ON usage_events FOR ALL USING (organization_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_policy_users ON users FOR ALL USING (organization_id = current_setting('app.current_tenant', true));

-- Handle tables with tenant_id (if any, like local_mcp_rag_tasks seen in sync)
-- Since they weren't matched in the exact migrations list above but are in code,
-- we'll add them if they exist using a PL/pgSQL block to be safe.
DO $$
DECLARE
    t text;
BEGIN
    FOR t IN
        SELECT table_name FROM information_schema.columns
        WHERE column_name = 'tenant_id'
        AND table_schema = 'public'
    LOOP
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY;', t);
        EXECUTE format('CREATE POLICY tenant_isolation_policy ON %I FOR ALL USING (tenant_id = current_setting(''app.current_tenant'', true));', t);
    END LOOP;
END $$;
