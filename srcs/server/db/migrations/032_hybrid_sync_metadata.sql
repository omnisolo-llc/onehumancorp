ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status TEXT DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
