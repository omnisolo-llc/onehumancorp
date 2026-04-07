-- +goose Up
-- +goose StatementBegin
ALTER TABLE agent_missions ADD COLUMN updated_at TIMESTAMP;
UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE updated_at IS NULL;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SQLite has limited support for dropping columns.
-- For Postgres, it would be ALTER TABLE agent_missions DROP COLUMN updated_at;
-- Since this is just adding a telemetry timestamp, down migration is omitted for SQLite cross-compatibility.
-- +goose StatementEnd