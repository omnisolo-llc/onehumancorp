-- +goose Up
ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
ALTER TABLE agent_memories DROP COLUMN last_sync_at;
ALTER TABLE agent_memories DROP COLUMN sync_status;
