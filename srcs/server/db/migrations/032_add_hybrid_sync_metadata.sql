-- 032_add_hybrid_sync_metadata.sql
-- Adds sync metadata for Hybrid MCP RAG Protocol.

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
