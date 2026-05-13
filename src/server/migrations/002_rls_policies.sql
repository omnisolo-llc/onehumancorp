-- 5. RLS Policies
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tenants ON tenants USING (id::text = current_setting('app.current_tenant', true));

ALTER TABLE users ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE knowledge_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_knowledge_embeddings ON knowledge_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_roles ON roles USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_order_items ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true));
