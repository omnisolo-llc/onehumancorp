-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN locked_until TIMESTAMP;
CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS idx_shared_tasks_locked_until;
DROP INDEX IF EXISTS idx_shared_tasks_status;
ALTER TABLE shared_tasks DROP COLUMN locked_until;
-- +goose StatementEnd
