-- +goose Up
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
-- +goose Down
-- Down migrations for SQLite drops are complex, usually omitted. For Postgres:
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN last_sync_at;
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_status;
