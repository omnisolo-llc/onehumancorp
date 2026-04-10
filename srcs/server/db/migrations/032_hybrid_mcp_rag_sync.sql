-- +goose Up
-- Add sync_status and last_sync_at to autodream_memories table
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

-- +goose Down
-- Dropping columns requires table recreation in SQLite, so we avoid it if possible,
-- but standard goose down would be:
-- ALTER TABLE autodream_memories DROP COLUMN sync_status;
-- ALTER TABLE autodream_memories DROP COLUMN last_sync_at;
