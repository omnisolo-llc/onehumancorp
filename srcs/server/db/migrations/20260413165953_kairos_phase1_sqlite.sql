-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    agent_id TEXT,
    status TEXT,
    payload TEXT,
    created_at DATETIME
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_tasks;
-- +goose StatementEnd
