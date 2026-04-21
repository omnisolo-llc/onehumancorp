-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS ohc_tasks (
    id VARCHAR PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    priority INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS ohc_tasks;
-- +goose StatementEnd
