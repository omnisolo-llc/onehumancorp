-- +goose Up
ALTER TABLE agent_missions ADD COLUMN sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN last_synced_at TIMESTAMP WITH TIME ZONE NULL;

-- +goose Down
ALTER TABLE agent_missions DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE agent_missions DROP COLUMN IF EXISTS sync_error;
