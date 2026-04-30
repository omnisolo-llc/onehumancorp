CREATE TABLE IF NOT EXISTS agent_approvals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(255) NOT NULL,
    task_id VARCHAR(255) NOT NULL,
    agent_id VARCHAR(255) NOT NULL,
    action_risk VARCHAR(50) NOT NULL,
    proposed_content TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_agent_approvals_tenant_id ON agent_approvals(tenant_id);
CREATE INDEX idx_agent_approvals_task_id ON agent_approvals(task_id);

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true));
