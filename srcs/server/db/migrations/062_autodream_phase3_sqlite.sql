-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    task_id TEXT,
    content TEXT NOT NULL,
    embedding TEXT,
    source_type TEXT NOT NULL DEFAULT 'auto_dream'
);

-- Ignore errors if column already exists
ALTER TABLE autodream_memories ADD COLUMN created_at DATETIME DEFAULT CURRENT_TIMESTAMP;
CREATE INDEX IF NOT EXISTS idx_autodream_memories_created_at ON autodream_memories (created_at);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS autodream_memories;
-- +goose StatementEnd
