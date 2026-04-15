-- +goose Up
-- +goose StatementBegin
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS autodream_memories_master (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_task_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS autodream_memories_master;
-- +goose StatementEnd
