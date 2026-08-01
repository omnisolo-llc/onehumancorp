-- +goose Up
CREATE TABLE IF NOT EXISTS agent_interaction_logs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    lead_id TEXT REFERENCES leads(id) ON DELETE SET NULL,
    customer_id TEXT,
    inbox_message_id TEXT,
    agent_type TEXT NOT NULL,
    action TEXT NOT NULL,
    context TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_interaction_logs_tenant_id ON agent_interaction_logs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_interaction_logs_lead_id ON agent_interaction_logs(lead_id);
CREATE INDEX IF NOT EXISTS idx_agent_interaction_logs_message_id ON agent_interaction_logs(inbox_message_id);

ALTER TABLE agent_interaction_logs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_interaction_logs ON agent_interaction_logs;
CREATE POLICY tenant_isolation_agent_interaction_logs
ON agent_interaction_logs
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_agent_interaction_logs ON agent_interaction_logs;
DROP TABLE IF EXISTS agent_interaction_logs CASCADE;
