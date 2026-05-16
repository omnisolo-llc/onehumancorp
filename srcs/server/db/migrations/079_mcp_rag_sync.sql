-- +goose Up
-- SQLite Schema additions for Standalone Agent
ALTER TABLE agent_missions ADD COLUMN synced_to_cloud BOOLEAN DEFAULT FALSE;
ALTER TABLE agent_missions ADD COLUMN cloud_mission_id TEXT;
ALTER TABLE agent_missions ADD COLUMN sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN last_synced_at TIMESTAMP;

-- +goose Down
ALTER TABLE agent_missions DROP COLUMN last_synced_at;
ALTER TABLE agent_missions DROP COLUMN sync_error;
ALTER TABLE agent_missions DROP COLUMN cloud_mission_id;
ALTER TABLE agent_missions DROP COLUMN synced_to_cloud;
