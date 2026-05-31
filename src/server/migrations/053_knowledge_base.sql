-- +goose Up
CREATE TABLE IF NOT EXISTS knowledge_base (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    content TEXT NOT NULL,
    metadata JSONB DEFAULT '{}',
    embedding VECTOR(1536)
);

ALTER TABLE knowledge_base ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_knowledge_base ON knowledge_base USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS knowledge_base_embedding_hnsw_idx ON knowledge_base USING hnsw (embedding vector_cosine_ops);

-- +goose Down
DROP TABLE IF EXISTS knowledge_base CASCADE;
