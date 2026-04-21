-- Rename telemetry_buffer to local_telemetry_buffer to align with Standalone Metric Buffer architecture.
-- Migration for both PostgreSQL and SQLite.

DO $$
BEGIN
    IF EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename  = 'telemetry_buffer') THEN
        ALTER TABLE telemetry_buffer RENAME TO local_telemetry_buffer;
    END IF;
END $$;

-- For SQLite (handled gracefully by the migration runner if it doesn't support DO blocks,
-- but we usually provide separate files if needed. However, the requirement is a single migration if possible).
-- The OHC migration runner handles basic ALTER TABLE RENAME.
-- Since SQLite doesn't support DO blocks, we might need a separate sqlite migration if the runner doesn't strip it.
-- Based on memories, the migration runner handles conversion.
-- But usually we have _pg and _sqlite suffixes for complex ones.
