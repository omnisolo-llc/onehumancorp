-- 053_harden_rls_policies.sql
-- Drop old vulnerable policies and create new hardened ones without the empty string check.

DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_swarm_memory ON swarm_memory;
CREATE POLICY tenant_isolation_swarm_memory ON swarm_memory USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_agent_status ON agent_status;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_capability_plugins ON capability_plugins;
CREATE POLICY tenant_isolation_capability_plugins ON capability_plugins USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings;
CREATE POLICY tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_usage_events ON usage_events;
CREATE POLICY tenant_isolation_usage_events ON usage_events USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_users ON users;
CREATE POLICY tenant_isolation_users ON users USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_scheduled_tasks ON scheduled_tasks;
CREATE POLICY tenant_isolation_scheduled_tasks ON scheduled_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_products ON products;
CREATE POLICY tenant_isolation_products ON products USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_agents ON agents;
CREATE POLICY tenant_isolation_agents ON agents USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_local_mcp_rag_tasks ON local_mcp_rag_tasks;
CREATE POLICY tenant_isolation_local_mcp_rag_tasks ON local_mcp_rag_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (
    task_id IN (SELECT id FROM shared_tasks WHERE organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system')
);

DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (
    meeting_id IN (
        SELECT id FROM meeting_rooms
        WHERE organization_id = current_setting('app.current_tenant', true)
           OR current_setting('app.current_tenant', true) = 'system'
    )
);
