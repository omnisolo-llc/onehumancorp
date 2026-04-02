ALTER TABLE embedding_cache ADD COLUMN synced_to_cloud BOOLEAN DEFAULT false;
ALTER TABLE agent_missions ADD COLUMN synced_to_cloud BOOLEAN DEFAULT false;
