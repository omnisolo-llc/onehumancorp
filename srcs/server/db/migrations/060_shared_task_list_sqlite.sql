-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_task_list_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    epic_id TEXT,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    payload TEXT,
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
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_task_list_dependencies;
DROP TABLE IF EXISTS shared_task_list_tasks;
-- +goose StatementEnd
