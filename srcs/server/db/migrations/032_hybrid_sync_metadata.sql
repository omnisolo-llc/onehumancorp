-- +goose Up
-- Add sync_status and last_sync_at to swarm_memory_embeddings
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- Remove sync_status and last_sync_at
-- SQLite doesn't directly support dropping columns nicely, but we provide it here for postgres context.
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_status;
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN last_sync_at;
