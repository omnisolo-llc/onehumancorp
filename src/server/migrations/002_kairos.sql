-- +goose Up
CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    agent_id TEXT,
    priority TEXT NOT NULL,
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL REFERENCES shared_tasks_v4(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- +goose Down
DROP TABLE IF EXISTS sub_agent_queue;
DROP TABLE IF EXISTS shared_tasks_v4;
