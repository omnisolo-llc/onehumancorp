-- 055_autodream_sync_status.sql
-- Add sync_status and last_sync_at to autodream_memories

ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP WITH TIME ZONE NULL;
