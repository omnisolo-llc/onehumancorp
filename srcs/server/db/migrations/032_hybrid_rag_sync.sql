-- +goose Up
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_timestamp TIMESTAMP NULL;

UPDATE autodream_memories SET sync_status = 'pending' WHERE sync_status IS NULL;

-- +goose Down
-- SQLite does not cleanly support dropping columns. Downward migrations often omitted.
