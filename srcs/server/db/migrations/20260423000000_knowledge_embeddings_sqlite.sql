-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS knowledge_embeddings (
    id TEXT PRIMARY KEY,
    content TEXT,
    metadata TEXT,
    embedding TEXT
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS knowledge_embeddings;
-- +goose StatementEnd
