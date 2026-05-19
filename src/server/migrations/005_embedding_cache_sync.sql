CREATE TABLE IF NOT EXISTS embedding_cache (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    synced_to_cloud BOOLEAN DEFAULT false
);

ALTER TABLE embedding_cache ADD COLUMN IF NOT EXISTS synced_to_cloud BOOLEAN DEFAULT false;
