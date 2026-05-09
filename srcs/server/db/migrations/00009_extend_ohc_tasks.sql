-- +goose Up
-- The design document requested a new table ohc_tasks:
-- id (UUID or TEXT, Primary Key)
-- title (TEXT, Not Null)
-- description (TEXT)
-- status (TEXT, e.g., 'PENDING', 'IN_PROGRESS', 'DONE', 'FAILED')
-- assigned_agent_id (TEXT, Nullable)
-- priority (INTEGER, Default 0)
-- created_at (TIMESTAMP)
-- updated_at (TIMESTAMP)

-- However, ohc_tasks already exists from 00004_ohc_tasks_parent_workflow.sql
-- We will alter it to match the requested design doc schema while keeping existing columns.

ALTER TABLE ohc_tasks ADD COLUMN title TEXT;
ALTER TABLE ohc_tasks ADD COLUMN description TEXT;
ALTER TABLE ohc_tasks ADD COLUMN priority INTEGER DEFAULT 0;

-- +goose Down
-- SQLite does not support drop column, but we will provide it for postgres.
ALTER TABLE ohc_tasks DROP COLUMN title;
ALTER TABLE ohc_tasks DROP COLUMN description;
ALTER TABLE ohc_tasks DROP COLUMN priority;