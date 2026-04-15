-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_tasks_master (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    agent_id VARCHAR,
    parent_plan_id VARCHAR,
    deliberation_log JSONB,
    dependencies JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies_master (
    task_id VARCHAR NOT NULL,
    depends_on_task_id VARCHAR NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS task_dependencies_master;
DROP TABLE IF EXISTS shared_tasks_master;
-- +goose StatementEnd
