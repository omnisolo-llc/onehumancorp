-- 032_hybrid_sync_metadata.sql
-- Migration to add sync metadata columns to swarm_memory_embeddings for Hybrid MCP RAG Protocol.
-- Keeping SQLite compatibility by adding columns individually.

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
