CREATE TABLE IF NOT EXISTS embedding_cache (
    id TEXT PRIMARY KEY,
    prompt TEXT NOT NULL,
    embedding VECTOR(1536),
    synced_to_cloud BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
