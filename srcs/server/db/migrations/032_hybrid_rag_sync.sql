-- +goose Up
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_status;
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN last_sync_at;
