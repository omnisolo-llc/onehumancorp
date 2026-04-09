-- Add sync metadata columns to autodream_memories for Hybrid MCP RAG Protocol
-- Using separate ALTER TABLE ADD COLUMN statements for SQLite compatibility

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
