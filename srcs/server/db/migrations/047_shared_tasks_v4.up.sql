-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    assigned_agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    locked_until TIMESTAMP WITH TIME ZONE,
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies_dag (
    task_id VARCHAR NOT NULL,
    depends_on_task_id VARCHAR NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);
-- +goose StatementEnd
