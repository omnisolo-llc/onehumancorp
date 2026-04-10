-- +goose Up
-- Add hybrid sync metadata to autodream_memories

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

UPDATE autodream_memories SET sync_status = 'synced' WHERE sync_status IS NULL;

CREATE INDEX IF NOT EXISTS idx_autodream_sync_status ON autodream_memories(sync_status);

-- +goose Down
-- Remove hybrid sync metadata
-- ALTER TABLE autodream_memories DROP COLUMN sync_status;
-- ALTER TABLE autodream_memories DROP COLUMN last_sync_at;
-- DROP INDEX IF EXISTS idx_autodream_sync_status;
