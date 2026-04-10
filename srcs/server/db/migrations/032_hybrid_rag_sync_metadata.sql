-- 032_hybrid_rag_sync_metadata.sql
-- Add hybrid sync metadata columns to swarm_memory_embeddings table

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_timestamp TIMESTAMP NULL;
