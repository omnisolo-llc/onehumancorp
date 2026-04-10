-- +goose Up
-- Add sync_status and last_sync_at to swarm_memory_embeddings
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

-- +goose Down
-- Remove sync_status and last_sync_at columns
-- Note: SQLite does not easily support DROP COLUMN in older versions.
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_status;
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN last_sync_at;
