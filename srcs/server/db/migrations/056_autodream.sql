-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS autodream_memories (
    id VARCHAR PRIMARY KEY,
    task_id VARCHAR REFERENCES shared_tasks_decomposition(id),
    content TEXT NOT NULL,
    embedding BLOB, -- Graceful degradation for SQLite/compatibility
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS autodream_memories;
-- +goose StatementEnd
