-- +goose Up
-- +goose StatementBegin
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON consolidated_memory
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE INDEX IF NOT EXISTS idx_consolidated_memory_embedding ON consolidated_memory USING hnsw (embedding vector_cosine_ops);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS consolidated_memory;
-- +goose StatementEnd
