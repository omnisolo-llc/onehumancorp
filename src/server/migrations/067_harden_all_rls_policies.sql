-- 067_harden_all_rls_policies.sql
-- Remove all fail-open RLS policies (e.g. `OR current_setting('app.current_tenant', true) = ''` or `= 'system'`)
-- and replace them with strictly bound policies to ensure unauthenticated session bypasses are prevented.

-- 1. Drop existing policies with `_t` suffix from 066_update_existing_rls_policies.sql
DROP POLICY IF EXISTS tenant_isolation_tasks_t ON tasks;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_t ON shared_tasks;
DROP POLICY IF EXISTS tenant_isolation_swarm_memory_t ON swarm_memory;
DROP POLICY IF EXISTS tenant_isolation_agent_missions_t ON agent_missions;
DROP POLICY IF EXISTS tenant_isolation_agent_status_t ON agent_status;
DROP POLICY IF EXISTS tenant_isolation_capability_plugins_t ON capability_plugins;
DROP POLICY IF EXISTS tenant_isolation_swarm_memory_embeddings_t ON swarm_memory_embeddings;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer_t ON telemetry_buffer;
DROP POLICY IF EXISTS tenant_isolation_usage_events_t ON usage_events;
DROP POLICY IF EXISTS tenant_isolation_users_t ON users;
DROP POLICY IF EXISTS tenant_isolation_scheduled_tasks_t ON scheduled_tasks;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories_t ON autodream_memories;
DROP POLICY IF EXISTS tenant_isolation_agent_memories_t ON agent_memories;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory_t ON consolidated_memory;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas_t ON crdt_deltas;
DROP POLICY IF EXISTS tenant_isolation_local_mcp_rag_tasks_t ON local_mcp_rag_tasks;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue_t ON sub_agent_queue;

DROP POLICY IF EXISTS tenant_isolation_tenants_t ON tenants;
DROP POLICY IF EXISTS tenant_isolation_products_t ON products;
DROP POLICY IF EXISTS tenant_isolation_orders_t ON orders;
DROP POLICY IF EXISTS tenant_isolation_customers_t ON customers;
DROP POLICY IF EXISTS tenant_isolation_bookings_t ON bookings;
DROP POLICY IF EXISTS tenant_isolation_agents_t ON agents;
DROP POLICY IF EXISTS tenant_isolation_pages_t ON pages;
DROP POLICY IF EXISTS tenant_isolation_memories_t ON memories;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies_t ON task_dependencies;

-- 2. Drop other policies with system fallback
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms_t ON meeting_rooms;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox_t ON agent_inbox;
DROP POLICY IF EXISTS tenant_isolation_knowledge_embeddings ON knowledge_embeddings;
DROP POLICY IF EXISTS tenant_isolation_tenant_calendars ON tenant_calendars;

-- Missing ones from old empty fallback
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;

-- Missing from 059_unified_schema_tenant_isolation.sql
DROP POLICY IF EXISTS tenant_isolation_order_items ON order_items;

DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
DROP POLICY IF EXISTS tenant_isolation_products ON products;
DROP POLICY IF EXISTS tenant_isolation_orders ON orders;
DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
DROP POLICY IF EXISTS tenant_isolation_agents ON agents;
DROP POLICY IF EXISTS tenant_isolation_pages ON pages;
DROP POLICY IF EXISTS tenant_isolation_memories ON memories;

DROP POLICY IF EXISTS tenant_isolation_knowledge_embeddings ON knowledge_embeddings;
DROP POLICY IF EXISTS tenant_isolation_tenant_calendars ON tenant_calendars;

-- from 065_update_rls_for_missing_tenant_id.sql
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms_t ON meeting_rooms;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox_t ON agent_inbox;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts_t ON meeting_transcripts;


-- 3. Create strictly bound policies without system/empty fallbacks
CREATE POLICY tenant_isolation_tasks_strict ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_shared_tasks_strict ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_memory_strict ON swarm_memory USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_missions_strict ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_status_strict ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_capability_plugins_strict ON capability_plugins USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_memory_embeddings_strict ON swarm_memory_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_telemetry_buffer_strict ON telemetry_buffer USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_usage_events_strict ON usage_events USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_users_strict ON users USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_scheduled_tasks_strict ON scheduled_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_autodream_memories_strict ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_memories_strict ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_consolidated_memory_strict ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_crdt_deltas_strict ON crdt_deltas USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_local_mcp_rag_tasks_strict ON local_mcp_rag_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_sub_agent_queue_strict ON sub_agent_queue USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_tenants_strict ON tenants USING (id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_products_strict ON products USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_orders_strict ON orders USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_customers_strict ON customers USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_bookings_strict ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agents_strict ON agents USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_pages_strict ON pages USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_memories_strict ON memories USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_task_dependencies_strict ON task_dependencies USING (task_id::text IN (SELECT id::text FROM shared_tasks WHERE tenant_id::text = current_setting('app.current_tenant', true)));
CREATE POLICY tenant_isolation_knowledge_embeddings_strict ON knowledge_embeddings USING (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_tenant_calendars_strict ON tenant_calendars USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_meeting_rooms_strict ON meeting_rooms USING (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_inbox_strict ON agent_inbox USING (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_tool_integrations_strict ON tool_integrations USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_order_items_strict ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_meeting_transcripts_strict ON meeting_transcripts USING (tenant_id::text = current_setting('app.current_tenant', true));
