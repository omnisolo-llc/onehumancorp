-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS epic_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS priority TEXT;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS priority;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS epic_id;
-- +goose StatementEnd
