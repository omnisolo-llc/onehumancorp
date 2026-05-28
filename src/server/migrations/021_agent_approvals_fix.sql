-- Create table agent_approvals if not exists, but we know it might exist. Ensure we have the columns.
CREATE TABLE IF NOT EXISTS agent_approvals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    department TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING_APPROVAL',
    action_risk TEXT NOT NULL,
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_approvals ON agent_approvals;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true));
