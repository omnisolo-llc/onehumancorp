-- +goose Up
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- In standard goose formatting, we do alters for postgres, although sqlite doesn't easily support drop column.
ALTER TABLE autodream_memories DROP COLUMN sync_status;
ALTER TABLE autodream_memories DROP COLUMN last_sync_at;
