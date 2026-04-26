-- 046_autodream_pgvector.sql

CREATE TABLE IF NOT EXISTS ohc_memory_embeddings (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BLOB, -- Graceful degradation for SQLite/compatibility
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    source_task_id VARCHAR
);
