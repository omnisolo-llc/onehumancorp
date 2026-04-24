-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS memory_embeddings (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    vector_embedding TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS memory_embeddings;
-- +goose StatementEnd
