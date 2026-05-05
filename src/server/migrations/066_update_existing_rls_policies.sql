-- 066_update_existing_rls_policies.sql
-- We must standardise on tenant_id for RLS policies rather than organization_id
-- We must drop policies that incorrectly use `OR current_setting('app.current_tenant', true) = ''`

-- Add missing columns to support the new policies
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_memory ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agent_status ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE capability_plugins ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE usage_events ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE scheduled_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE crdt_deltas ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE local_mcp_rag_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS tenant_id TEXT;

-- Ensure tables from 059 have it too in case they were missed
ALTER TABLE agents ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE pages ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE memories ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE products ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE customers ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS tenant_id TEXT;


-- Backfill data safely
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'tasks' AND column_name = 'organization_id') THEN UPDATE tasks SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'shared_tasks' AND column_name = 'organization_id') THEN UPDATE shared_tasks SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'swarm_memory' AND column_name = 'organization_id') THEN UPDATE swarm_memory SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agent_missions' AND column_name = 'organization_id') THEN UPDATE agent_missions SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agent_status' AND column_name = 'organization_id') THEN UPDATE agent_status SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'capability_plugins' AND column_name = 'organization_id') THEN UPDATE capability_plugins SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'swarm_memory_embeddings' AND column_name = 'organization_id') THEN UPDATE swarm_memory_embeddings SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'telemetry_buffer' AND column_name = 'organization_id') THEN UPDATE telemetry_buffer SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'usage_events' AND column_name = 'organization_id') THEN UPDATE usage_events SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'organization_id') THEN UPDATE users SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'scheduled_tasks' AND column_name = 'organization_id') THEN UPDATE scheduled_tasks SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'autodream_memories' AND column_name = 'organization_id') THEN UPDATE autodream_memories SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agent_memories' AND column_name = 'organization_id') THEN UPDATE agent_memories SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'sub_agent_queue' AND column_name = 'organization_id') THEN UPDATE sub_agent_queue SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agents' AND column_name = 'organization_id') THEN UPDATE agents SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'pages' AND column_name = 'organization_id') THEN UPDATE pages SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'memories' AND column_name = 'organization_id') THEN UPDATE memories SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'products' AND column_name = 'organization_id') THEN UPDATE products SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'orders' AND column_name = 'organization_id') THEN UPDATE orders SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'customers' AND column_name = 'organization_id') THEN UPDATE customers SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'bookings' AND column_name = 'organization_id') THEN UPDATE bookings SET tenant_id = organization_id WHERE tenant_id IS NULL; END IF;
END $$;

-- Drop old policies completely
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
DROP POLICY IF EXISTS tenant_isolation_swarm_memory ON swarm_memory;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
DROP POLICY IF EXISTS tenant_isolation_agent_status ON agent_status;
DROP POLICY IF EXISTS tenant_isolation_capability_plugins ON capability_plugins;
DROP POLICY IF EXISTS tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
DROP POLICY IF EXISTS tenant_isolation_usage_events ON usage_events;
DROP POLICY IF EXISTS tenant_isolation_users ON users;
DROP POLICY IF EXISTS tenant_isolation_scheduled_tasks ON scheduled_tasks;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
DROP POLICY IF EXISTS tenant_isolation_local_mcp_rag_tasks ON local_mcp_rag_tasks;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;

DROP POLICY IF EXISTS tenant_isolation_tenants_t ON tenants;
DROP POLICY IF EXISTS tenant_isolation_products_t ON products;
DROP POLICY IF EXISTS tenant_isolation_orders_t ON orders;
DROP POLICY IF EXISTS tenant_isolation_customers_t ON customers;
DROP POLICY IF EXISTS tenant_isolation_bookings_t ON bookings;
DROP POLICY IF EXISTS tenant_isolation_agents_t ON agents;
DROP POLICY IF EXISTS tenant_isolation_pages_t ON pages;
DROP POLICY IF EXISTS tenant_isolation_memories_t ON memories;

DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;

-- Create new standardized tenant_id policies
CREATE POLICY tenant_isolation_tasks_t ON tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_shared_tasks_t ON shared_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_swarm_memory_t ON swarm_memory USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_missions_t ON agent_missions USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_status_t ON agent_status USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_capability_plugins_t ON capability_plugins USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_swarm_memory_embeddings_t ON swarm_memory_embeddings USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_telemetry_buffer_t ON telemetry_buffer USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_usage_events_t ON usage_events USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_users_t ON users USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_scheduled_tasks_t ON scheduled_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_autodream_memories_t ON autodream_memories USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_memories_t ON agent_memories USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_consolidated_memory_t ON consolidated_memory USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_crdt_deltas_t ON crdt_deltas USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_local_mcp_rag_tasks_t ON local_mcp_rag_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_sub_agent_queue_t ON sub_agent_queue USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

CREATE POLICY tenant_isolation_tenants_t ON tenants USING (id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_products_t ON products USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_orders_t ON orders USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_customers_t ON customers USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_bookings_t ON bookings USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agents_t ON agents USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_pages_t ON pages USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_memories_t ON memories USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

CREATE POLICY tenant_isolation_task_dependencies_t ON task_dependencies USING (task_id IN (SELECT id FROM shared_tasks WHERE tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system'));
