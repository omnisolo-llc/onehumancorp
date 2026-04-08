-- SQLite does not support ADD COLUMN IF NOT EXISTS or CREATE EXTENSION
-- Postgres does, but for SQLite compatibility in testing we use simple ADD COLUMN.

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
