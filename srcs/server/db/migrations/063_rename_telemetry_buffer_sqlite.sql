-- Robustly rename telemetry_buffer to local_telemetry_buffer for SQLite.
-- Since SQLite doesn't support IF EXISTS for table rename, we use a multi-step approach.

-- 1. Create a placeholder table if it doesn't exist, to ensure ALTER TABLE doesn't fail.
CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    metric_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT DEFAULT 'system'
);

-- 2. Rename the table.
-- If it was already renamed (e.g. on a rerun), this might fail,
-- but our migration runner handles "no such table" gracefully if we use a dummy?
-- Actually, the best way in our runner is to just let it fail or skip if already done.
-- Since we use RunMigrations and it tracks filenames, this will only run once.
-- However, for fresh installs, it will create telemetry_buffer then rename it to local_telemetry_buffer.

ALTER TABLE telemetry_buffer RENAME TO local_telemetry_buffer;
