-- Create swarm_memory_embeddings table if not exists (for tests that might run this without older migrations if missing)
CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT NOT NULL,
    vector_embedding BLOB,
    source_plugin TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT
);

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
