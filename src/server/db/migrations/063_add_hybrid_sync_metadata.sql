-- +goose Up
-- +goose StatementBegin
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP NULL;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE swarm_memory_embeddings DROP COLUMN IF EXISTS sync_status;
ALTER TABLE swarm_memory_embeddings DROP COLUMN IF EXISTS last_sync_at;
-- +goose StatementEnd
