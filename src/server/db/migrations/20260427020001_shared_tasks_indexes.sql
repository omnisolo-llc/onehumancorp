-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ;
CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS idx_shared_tasks_locked_until;
DROP INDEX IF EXISTS idx_shared_tasks_status;
-- Note: SQLite does not support dropping columns easily, so we leave it or require manual intervention for down migration.
-- +goose StatementEnd
