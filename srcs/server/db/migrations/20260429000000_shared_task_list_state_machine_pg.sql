-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_task_list_tasks (
    id UUID PRIMARY KEY ,
    epic_id VARCHAR,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    payload JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    locked_by VARCHAR,
    locked_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS shared_task_list_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE TABLE IF NOT EXISTS shared_task_list_state_machine_transitions (
    id UUID PRIMARY KEY,
    task_id UUID NOT NULL,
    from_state VARCHAR NOT NULL,
    to_state VARCHAR NOT NULL,
    triggered_by VARCHAR,
    transitioned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_task_list_state_machine_transitions;
DROP TABLE IF EXISTS shared_task_list_dependencies;
DROP TABLE IF EXISTS shared_task_list_tasks;
-- +goose StatementEnd
