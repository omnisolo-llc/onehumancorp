-- 032_add_hybrid_sync_metadata.sql
-- Add sync_status and last_sync_at to agent_memories for Hybrid MCP RAG Protocol.

ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
