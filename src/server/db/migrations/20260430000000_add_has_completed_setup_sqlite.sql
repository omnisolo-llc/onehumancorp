-- +goose Up
-- +goose StatementBegin
ALTER TABLE users ADD COLUMN IF NOT EXISTS has_completed_setup BOOLEAN NOT NULL DEFAULT FALSE;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SQLite has limited support for DROP COLUMN in older versions, but modern ones support it.
ALTER TABLE users DROP COLUMN IF EXISTS has_completed_setup;
-- +goose StatementEnd
