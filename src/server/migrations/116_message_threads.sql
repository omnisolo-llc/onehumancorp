CREATE TABLE IF NOT EXISTS message_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    source TEXT,
    priority TEXT,
    context TEXT,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_drafts (
    id TEXT PRIMARY KEY,
    message_thread_id TEXT NOT NULL REFERENCES message_threads(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    action_type TEXT,
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_message_threads_tenant_status ON message_threads(tenant_id, status);

ALTER TABLE message_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_message_threads ON message_threads;
CREATE POLICY tenant_isolation_message_threads ON message_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_drafts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_drafts ON agent_drafts;
CREATE POLICY tenant_isolation_agent_drafts ON agent_drafts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
