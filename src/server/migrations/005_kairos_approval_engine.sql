CREATE TABLE IF NOT EXISTS agent_pending_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT,
    agent_id TEXT,
    risk_level TEXT,
    payload JSONB,
    status TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE agent_pending_actions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_pending_actions ON agent_pending_actions USING (tenant_id::text = current_setting('app.current_tenant', true));
