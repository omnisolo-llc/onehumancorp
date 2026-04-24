-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
    id TEXT PRIMARY KEY,
    parent_task_id TEXT,
    epic_id TEXT,
    organization_id TEXT,
    title TEXT,
    description TEXT,
    status TEXT,
    assigned_agent_id TEXT,
    priority TEXT,
    payload TEXT NOT NULL DEFAULT '{}',
    dependencies TEXT NOT NULL DEFAULT '{}',
    locked_until TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kairos_state_transitions (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    from_state TEXT,
    to_state TEXT,
    agent_id TEXT,
    reason TEXT,
    occurred_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
    id TEXT PRIMARY KEY,
    organization_id TEXT,
    parent_task_id TEXT,
    payload TEXT NOT NULL DEFAULT '{}',
    status TEXT,
    worker_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS autodream_vector_memories (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    content TEXT,
    embedding TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd
