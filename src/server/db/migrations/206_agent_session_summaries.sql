-- +goose Up
-- Migration: Add agent_session_summaries table for Episodic Memory Context Rehydration

CREATE TABLE IF NOT EXISTS agent_session_summaries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    customer_id TEXT,
    turn_index INTEGER NOT NULL DEFAULT 0,
    summary TEXT NOT NULL,
    summary_embedding vector(1536),
    raw_state JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS agent_session_summaries_tenant_id_idx ON agent_session_summaries(tenant_id);
CREATE INDEX IF NOT EXISTS agent_session_summaries_customer_id_idx ON agent_session_summaries(customer_id);
CREATE INDEX IF NOT EXISTS agent_session_summaries_embedding_idx ON agent_session_summaries USING hnsw (summary_embedding vector_cosine_ops);

ALTER TABLE agent_session_summaries ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'agent_session_summaries'
          AND policyname = 'tenant_isolation_agent_session_summaries'
    ) THEN
        CREATE POLICY tenant_isolation_agent_session_summaries ON agent_session_summaries
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_agent_session_summaries ON agent_session_summaries;
DROP TABLE IF EXISTS agent_session_summaries CASCADE;
