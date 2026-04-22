-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN deployment_mode TEXT NOT NULL DEFAULT 'standalone';
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SQLite does not support drop column if exists
-- +goose StatementEnd
