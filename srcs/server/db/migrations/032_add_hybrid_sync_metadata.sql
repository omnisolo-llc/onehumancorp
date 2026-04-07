-- +goose Up
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- Note: SQLite does not natively support dropping columns across all versions.
-- We keep the columns in older SQLite versions during rollback, but for Postgres we could drop them.
-- For compatibility and simplicity, downward migrations for adding columns are omitted.
