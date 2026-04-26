-- +goose Up
-- Add updated_at column to agent_missions
ALTER TABLE agent_missions ADD COLUMN updated_at TIMESTAMP;
UPDATE agent_missions SET updated_at = created_at WHERE updated_at IS NULL;

-- +goose Down
-- Remove updated_at column
-- SQLite doesn't easily support dropping columns directly in older versions,
-- but Postgres does. For cross-compat, downward migration usually relies on recreating the table
-- or is omitted if not fully necessary, but standard postgres drop:
-- ALTER TABLE agent_missions DROP COLUMN updated_at;