CREATE TABLE IF NOT EXISTS agent_session_summaries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    turn_index INTEGER NOT NULL,
    summary_embedding vector(1536),
    raw_state TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE agent_session_summaries ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON agent_session_summaries USING (tenant_id = current_setting('app.current_tenant'));
