-- +goose Up
ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- Dropping columns is not fully supported in old SQLite, but for completeness in PG:
-- ALTER TABLE agent_memories DROP COLUMN sync_status;
-- ALTER TABLE agent_memories DROP COLUMN last_sync_at;
