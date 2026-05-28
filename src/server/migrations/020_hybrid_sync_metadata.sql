ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP NULL;
