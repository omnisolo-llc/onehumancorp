-- 032_hybrid_sync_metadata.sql
-- Add sync_status and last_sync_at to swarm_memory_embeddings for Hybrid MCP RAG Protocol
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
