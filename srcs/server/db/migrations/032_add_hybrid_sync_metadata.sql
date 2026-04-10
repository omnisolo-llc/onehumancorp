-- Adds columns to autodream_memories for Hybrid MCP RAG Protocol sync status

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMPTZ;
