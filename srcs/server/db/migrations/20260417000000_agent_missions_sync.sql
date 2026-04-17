-- +goose Up
-- +goose StatementBegin
-- SQLite compatibility wrapper
ALTER TABLE agent_missions ADD COLUMN cloud_mission_id TEXT;
ALTER TABLE agent_missions ADD COLUMN sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN last_synced_at TIMESTAMP;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- Note: SQLite DROP COLUMN is limited in older versions.
-- +goose StatementEnd
