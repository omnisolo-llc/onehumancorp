ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMP NULL;
