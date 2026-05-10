-- +goose Up
-- +goose StatementBegin
-- +goose sqlite3
ALTER TABLE autodream_memories ADD COLUMN agent_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN source_type TEXT NOT NULL DEFAULT 'autodream';
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
CREATE EXTENSION IF NOT EXISTS vector;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS agent_id VARCHAR;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS source_type VARCHAR DEFAULT 'autodream';
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS agent_id;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS source_type;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- SQLite does not support DROP COLUMN easily before 3.35, but assuming standard here
ALTER TABLE autodream_memories DROP COLUMN agent_id;
ALTER TABLE autodream_memories DROP COLUMN source_type;
-- +goose StatementEnd
