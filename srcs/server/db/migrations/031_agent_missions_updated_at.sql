-- +goose Up
-- +goose StatementBegin
ALTER TABLE agent_missions ADD COLUMN updated_at TIMESTAMP;
UPDATE agent_missions SET updated_at = created_at WHERE updated_at IS NULL;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SQLite compatibility: skipping DROP COLUMN for downward migrations.
-- +goose StatementEnd
