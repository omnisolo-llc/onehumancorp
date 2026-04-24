-- +goose Up
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_enabled BOOLEAN DEFAULT FALSE;



-- +goose Down
ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_enabled;
