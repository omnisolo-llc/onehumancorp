-- +goose Up
-- +goose StatementBegin
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS knowledge_embeddings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE knowledge_embeddings ENABLE ROW LEVEL SECURITY;

-- Creating the IVFFlat index as required
CREATE INDEX IF NOT EXISTS idx_knowledge_embeddings_embedding ON knowledge_embeddings USING ivfflat (embedding vector_cosine_ops);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS knowledge_embeddings;
-- +goose StatementEnd
