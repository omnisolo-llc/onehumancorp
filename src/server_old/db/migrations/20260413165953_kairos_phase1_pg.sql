-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY,
    agent_id VARCHAR(255),
    status VARCHAR(50),
    payload JSONB,
    created_at TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_tasks;
-- +goose StatementEnd
