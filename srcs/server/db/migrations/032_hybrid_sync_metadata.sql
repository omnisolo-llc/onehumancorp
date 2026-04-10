-- 032_hybrid_sync_metadata.sql
-- Add hybrid sync metadata columns to swarm_memory_embeddings
-- Note: SQLite does not support ADD COLUMN IF NOT EXISTS.
-- It requires manual ALTER TABLE ADD COLUMN. Assuming it doesn't exist for now.

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
