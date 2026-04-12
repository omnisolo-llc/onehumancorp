CREATE TABLE IF NOT EXISTS ohc_memory_embeddings (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR,
    memory_type VARCHAR,
    content TEXT,
    embedding VECTOR(1536),
    source_task_id VARCHAR,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
