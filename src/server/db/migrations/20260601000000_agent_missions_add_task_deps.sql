-- +goose Up
-- Add task_id and dependencies to agent_missions
ALTER TABLE agent_missions ADD COLUMN task_id TEXT;
ALTER TABLE agent_missions ADD COLUMN dependencies TEXT;

-- +goose Down
-- Remove task_id and dependencies
-- SQLite does not support DROP COLUMN cleanly in all versions.
-- Postgres does:
-- ALTER TABLE agent_missions DROP COLUMN task_id;
-- ALTER TABLE agent_missions DROP COLUMN dependencies;
