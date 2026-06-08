CREATE TABLE IF NOT EXISTS agent_approvals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    department TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,
    action_risk TEXT NOT NULL,
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
