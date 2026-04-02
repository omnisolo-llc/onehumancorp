-- 011_add_synced_to_cloud_to_embedding.sql

ALTER TABLE embedding_cache ADD COLUMN synced_to_cloud BOOLEAN DEFAULT false;
