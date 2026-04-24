-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS ohc_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT,
    title TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    parent_task_id TEXT REFERENCES ohc_tasks(id),
    workflow_state TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS ohc_tasks;
-- +goose StatementEnd
