CREATE TABLE IF NOT EXISTS autodream_memories_master (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS autodream_memories_master_embedding_hnsw_idx ON autodream_memories_master USING hnsw (embedding vector_cosine_ops);
