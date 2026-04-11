-- 032_hybrid_mcp_rag_sync.sql
-- Hybrid MCP RAG Protocol: Bridging Standalone SQLite to Cloud PostgreSQL

ALTER TABLE swarm_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory ADD COLUMN last_sync_timestamp TIMESTAMPTZ NULL;
