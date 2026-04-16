-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS autodream_memories (id TEXT PRIMARY KEY);
ALTER TABLE autodream_memories ADD COLUMN processed_at TIMESTAMP WITH TIME ZONE;
CREATE INDEX IF NOT EXISTS idx_autodream_memories_processed_at ON autodream_memories(processed_at);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS idx_autodream_memories_processed_at;
-- SQLite does not fully support DROP COLUMN in older versions, but Goose handles it in modern SQLite versions.
CREATE TABLE IF NOT EXISTS autodream_memories (id TEXT PRIMARY KEY);
ALTER TABLE autodream_memories DROP COLUMN processed_at;
-- +goose StatementEnd
