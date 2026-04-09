-- 032_hybrid_sync_metadata.sql
-- Add hybrid sync metadata to swarm_memory_embeddings (the primary context table)

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
