-- +goose Up
-- +goose StatementBegin
CREATE SCHEMA IF NOT EXISTS ohc_memory;
CREATE TABLE IF NOT EXISTS ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID,
    content TEXT NOT NULL,
    embedding vector(1536),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS ohc_memory.autodream_vectors;
-- +goose StatementEnd
