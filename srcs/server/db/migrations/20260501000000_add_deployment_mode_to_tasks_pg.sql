-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS deployment_mode TEXT NOT NULL DEFAULT 'standalone';
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS deployment_mode;
-- +goose StatementEnd
