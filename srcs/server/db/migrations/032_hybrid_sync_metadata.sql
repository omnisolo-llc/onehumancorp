ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMP NULL;
CREATE INDEX IF NOT EXISTS idx_consolidated_memory_sync_status ON consolidated_memory(sync_status);
