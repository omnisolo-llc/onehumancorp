ALTER TABLE swarm_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory ADD COLUMN last_sync_at TIMESTAMP NULL;
