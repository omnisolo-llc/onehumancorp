CREATE TABLE IF NOT EXISTS agent_approvals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    department TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    action_risk TEXT NOT NULL,
    payload TEXT DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON agent_approvals USING (tenant_id = current_setting('app.current_tenant', true));
