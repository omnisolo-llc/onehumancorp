-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_task_list_tasks (
    id TEXT PRIMARY KEY ,
    epic_id TEXT,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    payload JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    locked_by TEXT,
    locked_at DATETIME
);

CREATE TABLE IF NOT EXISTS shared_task_list_dependencies (
    task_id TEXT NOT NULL,
    depends_on_task_id TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE TABLE IF NOT EXISTS shared_task_list_state_machine_transitions (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    triggered_by TEXT,
    transitioned_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_task_list_state_machine_transitions;
DROP TABLE IF EXISTS shared_task_list_dependencies;
DROP TABLE IF EXISTS shared_task_list_tasks;
-- +goose StatementEnd
