-- 059_vector_extension.sql

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (organization_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_consolidated_memory_embedding ON consolidated_memory USING hnsw (embedding vector_cosine_ops);
