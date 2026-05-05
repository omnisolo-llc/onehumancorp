-- +goose Up
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP NULL;

-- +goose Down
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS sync_status;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS last_sync_at;
