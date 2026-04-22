-- +goose Up
CREATE TABLE IF NOT EXISTS mcp_async_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- +goose Down
DROP TABLE IF EXISTS mcp_async_tasks;
