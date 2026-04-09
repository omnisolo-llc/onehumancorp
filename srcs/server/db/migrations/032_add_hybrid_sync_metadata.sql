-- 032_add_hybrid_sync_metadata.sql
-- Add hybrid sync tracking columns to swarm_memory_embeddings for the Hybrid MCP RAG Protocol.

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
