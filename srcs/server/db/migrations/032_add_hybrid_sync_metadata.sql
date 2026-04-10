-- +goose Up
ALTER TABLE swarm_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- ALTER TABLE swarm_memory DROP COLUMN sync_status;
-- ALTER TABLE swarm_memory DROP COLUMN last_sync_at;
