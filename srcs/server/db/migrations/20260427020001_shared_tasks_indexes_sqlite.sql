-- Add locked_until and indexes
-- SQLite doesn't support IF NOT EXISTS for columns in older versions, but we assume it might exist or not.
-- Since the migration script doesn't handle SQLite ALTER TABLE gracefully if it exists, let's just try to add it.
-- Actually, modern SQLite does support ADD COLUMN. If it fails, goose handles it.
-- Or we can skip adding the column if we know it was added in 20260416.
-- Let's just follow standard SQLite migration.
ALTER TABLE shared_tasks ADD COLUMN locked_until DATETIME;
CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until);
