CREATE TABLE IF NOT EXISTS agent_approvals (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    action_risk VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true));
