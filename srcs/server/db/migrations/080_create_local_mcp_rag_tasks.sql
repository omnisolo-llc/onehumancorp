-- +goose Up
-- +goose StatementBegin
CREATE TABLE local_mcp_rag_tasks (
    id TEXT PRIMARY KEY,
    payload TEXT NOT NULL,
    escalation_status TEXT NOT NULL DEFAULT 'local',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE local_mcp_rag_tasks;
-- +goose StatementEnd
