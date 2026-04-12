CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS ohc_memory_embeddings (
    id VARCHAR PRIMARY KEY,
    tenant_id TEXT,
    memory_type TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_task_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
