-- 032_hybrid_rag_sync.sql
-- Add hybrid sync tracking columns to swarm_memory_embeddings

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
