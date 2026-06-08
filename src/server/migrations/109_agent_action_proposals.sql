CREATE TABLE IF NOT EXISTS agent_action_proposals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_message_id TEXT NOT NULL REFERENCES inbox_messages(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_action_proposals_tenant_msg ON agent_action_proposals(tenant_id, inbox_message_id);

ALTER TABLE agent_action_proposals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_action_proposals ON agent_action_proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
