-- 011_autodream_sync.sql
-- Add synced_to_cloud column to embedding_cache

ALTER TABLE embedding_cache ADD COLUMN synced_to_cloud BOOLEAN DEFAULT false;
