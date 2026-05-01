-- +goose Up
-- Create shared_tasks schema for KAIROS Orchestration
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY,
    agent_id VARCHAR(255),
    status VARCHAR(50),
    payload JSONB,
    created_at TIMESTAMP
);

-- +goose Down
DROP TABLE IF EXISTS shared_tasks;
