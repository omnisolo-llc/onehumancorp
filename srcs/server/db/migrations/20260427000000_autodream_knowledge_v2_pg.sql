-- +goose Up
-- +goose StatementBegin
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS knowledge_embeddings (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_id TEXT,
    source_type TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_knowledge_embeddings_vector ON knowledge_embeddings USING ivfflat (embedding vector_cosine_ops);

ALTER TABLE swarm_ultra_plans ADD COLUMN IF NOT EXISTS auto_dreamed BOOLEAN DEFAULT FALSE;
ALTER TABLE shared_tasks_decomposition ADD COLUMN IF NOT EXISTS auto_dreamed BOOLEAN DEFAULT FALSE;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks_decomposition DROP COLUMN IF EXISTS auto_dreamed;
ALTER TABLE swarm_ultra_plans DROP COLUMN IF EXISTS auto_dreamed;
DROP TABLE IF EXISTS knowledge_embeddings;
-- +goose StatementEnd
