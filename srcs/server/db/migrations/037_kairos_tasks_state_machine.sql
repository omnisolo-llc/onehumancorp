-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN parent_task_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN workflow_state TEXT;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN parent_task_id;
ALTER TABLE shared_tasks DROP COLUMN workflow_state;
-- +goose StatementEnd
