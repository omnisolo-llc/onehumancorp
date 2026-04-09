-- 032_add_hybrid_sync_metadata.sql
-- Add hybrid sync metadata columns to swarm_memory_embeddings
-- SQLite doesn't support adding multiple columns in one ALTER TABLE statement

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
