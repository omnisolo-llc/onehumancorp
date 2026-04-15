-- +goose Up
-- +goose StatementBegin
CREATE TABLE swarm_tasks (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    priority VARCHAR(50) NOT NULL DEFAULT 'P0',
    agent_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE state_machine_transitions (
    id UUID PRIMARY KEY,
    task_id UUID NOT NULL,
    from_state VARCHAR(50),
    to_state VARCHAR(50) NOT NULL,
    triggered_by VARCHAR(255),
    transitioned_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE
);

CREATE TABLE task_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id),
    FOREIGN KEY (task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (depends_on_task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS task_dependencies;
DROP TABLE IF EXISTS state_machine_transitions;
DROP TABLE IF EXISTS swarm_tasks;
-- +goose StatementEnd
