-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
    id VARCHAR PRIMARY KEY,
    parent_task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    epic_id VARCHAR,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    assigned_agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    locked_until TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kairos_state_transitions (
    id VARCHAR PRIMARY KEY,
    task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    from_state VARCHAR NOT NULL,
    to_state VARCHAR NOT NULL,
    agent_id VARCHAR NOT NULL,
    reason TEXT,
    occurred_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    parent_task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    payload TEXT,
    status VARCHAR NOT NULL DEFAULT 'QUEUED',
    worker_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS autodream_vector_memories (
    id VARCHAR PRIMARY KEY,
    task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    content TEXT NOT NULL,
    embedding TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS autodream_vector_memories;
DROP TABLE IF EXISTS kairos_sub_agent_jobs;
DROP TABLE IF EXISTS kairos_state_transitions;
DROP TABLE IF EXISTS kairos_shared_tasks;
-- +goose StatementEnd
