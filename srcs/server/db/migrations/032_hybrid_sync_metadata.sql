-- +goose Up
-- Add sync_status and last_sync_at to agent_memories
ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

-- +goose Down
-- ALTER TABLE agent_memories DROP COLUMN sync_status;
-- ALTER TABLE agent_memories DROP COLUMN last_sync_at;
