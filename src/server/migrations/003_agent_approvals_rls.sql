ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true));
