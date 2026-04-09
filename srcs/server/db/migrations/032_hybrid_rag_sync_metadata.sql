-- 032_hybrid_rag_sync_metadata.sql
-- Adds sync status and timestamp to agent_memories for Hybrid MCP RAG Protocol.
-- Uses standard SQL compatible with both PostgreSQL and SQLite.

ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
