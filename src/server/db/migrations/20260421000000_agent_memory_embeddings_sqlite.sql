-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS agent_memory_embeddings (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    organization_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL DEFAULT '',
    agent_id TEXT NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS agent_memory_embeddings;
-- +goose StatementEnd
