-- +goose Up
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

-- +goose Down
-- SQLite doesn't easily support dropping columns without recreating the table,
-- but for completeness (especially for PostgreSQL):
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS sync_status;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS last_sync_at;
