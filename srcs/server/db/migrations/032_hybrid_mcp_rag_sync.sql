-- 032_hybrid_mcp_rag_sync.sql
-- Add sync metadata to swarm_memory tables for Hybrid MCP RAG Protocol

ALTER TABLE swarm_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
