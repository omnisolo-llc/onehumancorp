CREATE TABLE IF NOT EXISTS agent_feed_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    event_source TEXT NOT NULL,
    context_payload JSONB,
    proposed_action JSONB,
    lifecycle_state TEXT NOT NULL DEFAULT 'PENDING_APPROVAL',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE agent_feed_items ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_agent_feed_items ON agent_feed_items
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS agent_feed_items_tenant_state_idx ON agent_feed_items(tenant_id, lifecycle_state);
CREATE INDEX IF NOT EXISTS agent_feed_items_tenant_created_idx ON agent_feed_items(tenant_id, created_at DESC);
