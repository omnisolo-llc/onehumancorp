-- +goose Up
-- Add sync_status and last_sync_at to autodream_memories
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- Remove sync_status and last_sync_at
-- ALTER TABLE autodream_memories DROP COLUMN sync_status;
-- ALTER TABLE autodream_memories DROP COLUMN last_sync_at;
