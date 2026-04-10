ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_timestamp TIMESTAMP NULL;

UPDATE autodream_memories SET sync_status = 'pending' WHERE sync_status IS NULL;

CREATE INDEX IF NOT EXISTS idx_autodream_sync_status ON autodream_memories(sync_status);
