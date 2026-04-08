-- SQLite does not support ADD COLUMN IF NOT EXISTS or CREATE EXTENSION
-- Postgres does, but for SQLite compatibility in testing we use simple ADD COLUMN.
-- However, SQLite will error if the column already exists.
-- Since this is an unreleased migration in our local environment, we can just drop/recreate,
-- or use proper SQLite compatible syntax.

-- Because we just added autodream_memories in 024 and altered it in 029, let's just alter it simply.
-- Actually, SQLite ALTER TABLE ADD COLUMN does not support IF NOT EXISTS.

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
