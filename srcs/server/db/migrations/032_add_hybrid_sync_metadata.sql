-- +goose Up
-- Add hybrid sync metadata to agent_memories
-- SQLite compatible separate ADD COLUMN statements
ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMP;

-- +goose Down
-- ALTER TABLE agent_memories DROP COLUMN sync_status;
-- ALTER TABLE agent_memories DROP COLUMN last_sync_at;
