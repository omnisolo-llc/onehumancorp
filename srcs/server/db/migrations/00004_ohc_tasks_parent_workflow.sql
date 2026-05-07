-- +goose Up
CREATE TABLE IF NOT EXISTS ohc_tasks (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    payload JSONB,
    assigned_agent_id VARCHAR,
    parent_task_id TEXT,
    workflow_state TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE ohc_tasks ADD COLUMN IF NOT EXISTS parent_task_id TEXT;
ALTER TABLE ohc_tasks ADD COLUMN IF NOT EXISTS workflow_state TEXT;

-- +goose Down
-- DROP TABLE ohc_tasks;
