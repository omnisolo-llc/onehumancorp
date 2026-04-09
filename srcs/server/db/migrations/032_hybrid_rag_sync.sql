-- +goose Up
-- Add sync_status and last_sync_at to autodream_memories for Hybrid RAG Sync
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- Update existing rows (if any) to have 'pending' status just to be safe
UPDATE autodream_memories SET sync_status = 'pending' WHERE sync_status IS NULL;

-- +goose Down
-- Down migrations generally not used for SQLite compatibility but included for Postgres
-- ALTER TABLE autodream_memories DROP COLUMN sync_status;
-- ALTER TABLE autodream_memories DROP COLUMN last_sync_at;
