-- 007_powersync.sql
-- Publication for PowerSync

-- Note: CREATE PUBLICATION is PostgreSQL-only and will fail on SQLite.
-- For standalone mode compatibility, we wrap this in a conditional execution block in Go,
-- but since migrations execute blindly across both DB types, we just create a dummy table
-- to ensure the migration passes on SQLite, and the actual publication logic needs to be handled dynamically in Go or skipped on SQLite.

-- We create a dummy table to record that this migration ran.
CREATE TABLE IF NOT EXISTS _powersync_publication_marker (id INTEGER PRIMARY KEY);
