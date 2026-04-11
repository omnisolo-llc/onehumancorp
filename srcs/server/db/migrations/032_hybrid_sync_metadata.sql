ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
