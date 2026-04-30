-- 049_rls_policies.sql
-- Ensure RLS policies are enforced on all multi-tenant tables using 'current_setting('app.current_tenant', true)'

-- Policy for tasks
DROP POLICY IF EXISTS tenant_isolation_policy ON tasks;
CREATE POLICY tenant_isolation_policy ON tasks
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for shared_tasks
DROP POLICY IF EXISTS tenant_isolation_policy ON shared_tasks;
CREATE POLICY tenant_isolation_policy ON shared_tasks
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for swarm_memory
DROP POLICY IF EXISTS tenant_isolation_policy ON swarm_memory;
CREATE POLICY tenant_isolation_policy ON swarm_memory
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for agent_missions
DROP POLICY IF EXISTS tenant_isolation_policy ON agent_missions;
CREATE POLICY tenant_isolation_policy ON agent_missions
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for agent_status
DROP POLICY IF EXISTS tenant_isolation_policy ON agent_status;
CREATE POLICY tenant_isolation_policy ON agent_status
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for capability_plugins
DROP POLICY IF EXISTS tenant_isolation_policy ON capability_plugins;
CREATE POLICY tenant_isolation_policy ON capability_plugins
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for swarm_memory_embeddings
DROP POLICY IF EXISTS tenant_isolation_policy ON swarm_memory_embeddings;
CREATE POLICY tenant_isolation_policy ON swarm_memory_embeddings
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for telemetry_buffer
DROP POLICY IF EXISTS tenant_isolation_policy ON telemetry_buffer;
CREATE POLICY tenant_isolation_policy ON telemetry_buffer
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for usage_events
DROP POLICY IF EXISTS tenant_isolation_policy ON usage_events;
CREATE POLICY tenant_isolation_policy ON usage_events
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for users
DROP POLICY IF EXISTS tenant_isolation_policy ON users;
CREATE POLICY tenant_isolation_policy ON users
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for meeting_rooms
DROP POLICY IF EXISTS tenant_isolation_policy ON meeting_rooms;
CREATE POLICY tenant_isolation_policy ON meeting_rooms
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for agent_inbox
DROP POLICY IF EXISTS tenant_isolation_policy ON agent_inbox;
CREATE POLICY tenant_isolation_policy ON agent_inbox
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for scheduled_tasks
DROP POLICY IF EXISTS tenant_isolation_policy ON scheduled_tasks;
CREATE POLICY tenant_isolation_policy ON scheduled_tasks
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for autodream_memories
DROP POLICY IF EXISTS tenant_isolation_policy ON autodream_memories;
CREATE POLICY tenant_isolation_policy ON autodream_memories
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for agent_memories
DROP POLICY IF EXISTS tenant_isolation_policy ON agent_memories;
CREATE POLICY tenant_isolation_policy ON agent_memories
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for consolidated_memory
DROP POLICY IF EXISTS tenant_isolation_policy ON consolidated_memory;
CREATE POLICY tenant_isolation_policy ON consolidated_memory
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for crdt_deltas
DROP POLICY IF EXISTS tenant_isolation_policy ON crdt_deltas;
CREATE POLICY tenant_isolation_policy ON crdt_deltas
    AS RESTRICTIVE
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Policy for local_mcp_rag_tasks
DROP POLICY IF EXISTS tenant_isolation_policy ON local_mcp_rag_tasks;
CREATE POLICY tenant_isolation_policy ON local_mcp_rag_tasks
    AS RESTRICTIVE
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Policy for agents
DROP POLICY IF EXISTS tenant_isolation_policy ON agents;
CREATE POLICY tenant_isolation_policy ON agents
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

-- Policy for products
DROP POLICY IF EXISTS tenant_isolation_policy ON products;
CREATE POLICY tenant_isolation_policy ON products
    AS RESTRICTIVE
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));
