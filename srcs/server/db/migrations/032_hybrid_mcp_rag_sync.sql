ALTER TABLE autodream_memories ADD COLUMN sync_status TEXT DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_timestamp TIMESTAMPTZ NULL;
