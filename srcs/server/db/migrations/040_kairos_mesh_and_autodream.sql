-- +goose Up
-- +goose StatementBegin
CREATE INDEX IF NOT EXISTS idx_shared_tasks_polling ON shared_tasks(status, priority, created_at);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS idx_shared_tasks_polling;
-- +goose StatementEnd
