-- +goose Up
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- Dropping columns is not supported in SQLite, so Down is empty to maintain compatibility.
