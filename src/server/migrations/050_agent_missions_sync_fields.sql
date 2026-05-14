ALTER TABLE agent_missions ADD COLUMN cloud_mission_id TEXT; ALTER TABLE agent_missions ADD COLUMN sync_error TEXT; ALTER TABLE agent_missions ADD COLUMN last_synced_at TIMESTAMPTZ;
