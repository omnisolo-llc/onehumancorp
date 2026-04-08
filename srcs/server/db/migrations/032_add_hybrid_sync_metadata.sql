-- In SQLite, altering table to add column with a default value of 'pending' can be tricky, but standard is `ALTER TABLE table_name ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending'`.
-- And `ALTER TABLE table_name ADD COLUMN last_sync_at TIMESTAMP NULL`.
-- Let's apply this to `autodream_memories` because it's the primary memory/RAG context table based on the KAIROS architecture.

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
