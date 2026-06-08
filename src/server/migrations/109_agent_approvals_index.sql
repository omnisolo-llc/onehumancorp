CREATE INDEX IF NOT EXISTS idx_agent_approvals_tenant_status_id ON agent_approvals(tenant_id, status, id DESC);
