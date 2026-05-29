ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS synced_to_cloud BOOLEAN DEFAULT false;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMPTZ;
