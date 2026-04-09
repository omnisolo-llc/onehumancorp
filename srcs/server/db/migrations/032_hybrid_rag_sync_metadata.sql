-- 032_hybrid_rag_sync_metadata.sql
-- Add sync metadata columns to swarm_memory for the Hybrid MCP RAG Protocol

ALTER TABLE swarm_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory ADD COLUMN last_sync_at TIMESTAMP NULL;
