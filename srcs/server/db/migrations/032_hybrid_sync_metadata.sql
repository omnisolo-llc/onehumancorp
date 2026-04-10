-- 032_hybrid_sync_metadata.sql
-- Add synchronization metadata to swarm_memory_embeddings for Hybrid MCP RAG Protocol

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_timestamp TIMESTAMP NULL;
