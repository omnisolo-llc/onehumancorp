-- +goose Up
-- +goose sqlite3
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS local_mcp_rag_tasks (
    id TEXT PRIMARY KEY,
    task_data TEXT NOT NULL,
    escalation_status TEXT NOT NULL DEFAULT 'local',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose postgres
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS local_mcp_rag_tasks (
    id UUID PRIMARY KEY,
    task_data TEXT NOT NULL,
    escalation_status VARCHAR(50) NOT NULL DEFAULT 'local',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
DROP TABLE IF EXISTS local_mcp_rag_tasks;
