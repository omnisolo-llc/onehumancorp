-- 032_add_hybrid_sync_metadata.sql
-- Migration to support Hybrid MCP RAG Protocol (Standalone SQLite <-> Cloud PostgreSQL sync)

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
