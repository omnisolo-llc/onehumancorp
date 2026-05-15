ALTER TABLE roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_roles ON roles USING (tenant_id::text = current_setting('app.current_tenant', true) OR tenant_id = 'system');

ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true) OR tenant_id = 'system');

ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_order_items ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true));
