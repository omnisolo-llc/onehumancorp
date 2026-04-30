-- 048_rls_policies.sql
-- Implement RLS Policies for Tenant Isolation

CREATE POLICY tasks_isolation_policy ON tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY shared_tasks_isolation_policy ON shared_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY swarm_memory_isolation_policy ON swarm_memory USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY agent_missions_isolation_policy ON agent_missions USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY agent_status_isolation_policy ON agent_status USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY capability_plugins_isolation_policy ON capability_plugins USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY swarm_memory_embeddings_isolation_policy ON swarm_memory_embeddings USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY telemetry_buffer_isolation_policy ON telemetry_buffer USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY usage_events_isolation_policy ON usage_events USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY users_isolation_policy ON users USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY meeting_rooms_isolation_policy ON meeting_rooms USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY agent_inbox_isolation_policy ON agent_inbox USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY scheduled_tasks_isolation_policy ON scheduled_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY autodream_memories_isolation_policy ON autodream_memories USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY agent_memories_isolation_policy ON agent_memories USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY consolidated_memory_isolation_policy ON consolidated_memory USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY crdt_deltas_isolation_policy ON crdt_deltas USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY local_mcp_rag_tasks_isolation_policy ON local_mcp_rag_tasks USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
