CREATE TABLE IF NOT EXISTS agent_approvals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    department TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,
    action_risk TEXT,
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_approvals_tenant_status ON agent_approvals(tenant_id, status);

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
