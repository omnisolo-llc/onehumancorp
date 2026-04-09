-- 032_hybrid_sync_metadata.sql

-- Add sync_status to autodream_memories
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';

-- Add last_sync_at to autodream_memories
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
