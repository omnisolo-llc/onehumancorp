-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd
