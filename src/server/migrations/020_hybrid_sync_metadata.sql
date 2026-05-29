CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT NOT NULL,
    vector_embedding BYTEA,
    source_plugin TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT
);

ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP NULL;
