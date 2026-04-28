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

DO $$
DECLARE
    t text;
BEGIN
    FOR t IN
        SELECT table_name FROM information_schema.columns
        WHERE column_name = 'tenant_id'
        AND table_schema = 'public'
    LOOP
        EXECUTE format('CREATE POLICY tenant_isolation_policy ON %I FOR ALL USING (current_setting(''app.current_tenant'', true) = ''system'' OR tenant_id::text = current_setting(''app.current_tenant'', true));', t);
    END LOOP;
END $$;
