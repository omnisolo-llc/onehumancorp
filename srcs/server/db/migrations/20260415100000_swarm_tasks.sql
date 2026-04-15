-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS swarm_tasks (
    id VARCHAR PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    priority VARCHAR,
    agent_id VARCHAR,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id VARCHAR PRIMARY KEY,
    task_id VARCHAR NOT NULL,
    from_state VARCHAR NOT NULL,
    to_state VARCHAR NOT NULL,
    triggered_by VARCHAR,
    transitioned_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id VARCHAR NOT NULL,
    depends_on_task_id VARCHAR NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS task_dependencies;
DROP TABLE IF EXISTS state_machine_transitions;
DROP TABLE IF EXISTS swarm_tasks;
-- +goose StatementEnd
