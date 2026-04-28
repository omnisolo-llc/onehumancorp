-- 044_autodream_sync_cloud.sql
-- Add synced_to_cloud column to embedding_cache and agent_missions
-- Note: Re-adding as a new migration script to ensure the column is properly created
-- if the previous migration wasn't executed or tracked correctly.

ALTER TABLE embedding_cache ADD COLUMN synced_to_cloud BOOLEAN DEFAULT false;
ALTER TABLE agent_missions ADD COLUMN synced_to_cloud BOOLEAN DEFAULT false;
