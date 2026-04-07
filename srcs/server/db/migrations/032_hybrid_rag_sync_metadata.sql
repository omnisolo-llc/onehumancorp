-- 032_hybrid_rag_sync_metadata.sql
-- Adds sync tracking columns to the swarm_memory_embeddings table for Hybrid MCP RAG Protocol.

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
