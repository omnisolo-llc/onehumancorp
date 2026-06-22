CREATE TABLE IF NOT EXISTS agent_drafts (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    work_item_id VARCHAR NOT NULL,
    proposed_action JSONB NOT NULL,
    context JSONB NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_drafts_tenant_status ON agent_drafts (tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_agent_drafts_work_item ON agent_drafts (work_item_id);

ALTER TABLE agent_drafts ENABLE ROW LEVEL SECURITY;

CREATE POLICY agent_drafts_tenant_isolation_policy ON agent_drafts
    USING (tenant_id = current_setting('app.current_tenant', true));
