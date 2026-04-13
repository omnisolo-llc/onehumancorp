-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN parent_task_id VARCHAR;
ALTER TABLE shared_tasks ADD COLUMN workflow_state JSONB;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN parent_task_id;
ALTER TABLE shared_tasks DROP COLUMN workflow_state;
-- +goose StatementEnd
