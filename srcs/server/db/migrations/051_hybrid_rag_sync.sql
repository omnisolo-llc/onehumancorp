-- +goose Up
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_enabled BOOLEAN DEFAULT FALSE;
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMPTZ NULL;

-- +goose Down
ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_enabled;
