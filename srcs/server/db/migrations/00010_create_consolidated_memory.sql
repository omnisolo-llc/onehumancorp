-- +goose Up
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    agent_id VARCHAR,
    task_id VARCHAR,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type VARCHAR NOT NULL DEFAULT 'autodream',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (organization_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_consolidated_memory_embedding ON consolidated_memory USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
CREATE INDEX IF NOT EXISTS idx_consolidated_memory_org ON consolidated_memory(organization_id);

-- +goose Down
DROP INDEX IF EXISTS idx_consolidated_memory_org;
DROP INDEX IF EXISTS idx_consolidated_memory_embedding;
DROP TABLE IF EXISTS consolidated_memory;
