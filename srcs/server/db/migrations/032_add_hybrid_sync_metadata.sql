-- 032_add_hybrid_sync_metadata.sql
-- Adds synchronization metadata columns to swarm_memory_embeddings for the Hybrid MCP RAG Protocol.

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50);
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ;

UPDATE swarm_memory_embeddings SET sync_status = 'pending' WHERE sync_status IS NULL;
