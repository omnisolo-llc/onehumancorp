-- +goose Up
-- +goose StatementBegin
ALTER TABLE agent_missions ADD COLUMN updated_at DATETIME;
UPDATE agent_missions SET updated_at = created_at WHERE updated_at IS NULL;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SQLite does not support dropping columns natively.
-- Postgres could drop the column, but for cross-compatibility we skip this in down migrations.
-- +goose StatementEnd
