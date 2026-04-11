-- 032_add_hybrid_sync_metadata.sql
-- Add synchronization metadata to Swarm Intelligence Protocol memory tables for Hybrid MCP RAG

ALTER TABLE swarm_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
