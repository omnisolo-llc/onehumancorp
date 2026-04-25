-- +goose Up
-- +goose StatementBegin
-- SQLite doesn't support RLS, so this is a no-op
SELECT 1;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 1;
-- +goose StatementEnd
