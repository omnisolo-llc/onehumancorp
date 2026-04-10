-- 032_add_hybrid_sync_metadata.sql
-- Adds sync_status and last_sync_at to autodream_memories for the Hybrid MCP RAG Protocol

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- Set default values for existing rows
UPDATE autodream_memories SET sync_status = 'synced' WHERE sync_status IS NULL;

CREATE INDEX IF NOT EXISTS idx_autodream_sync_status ON autodream_memories(sync_status);
