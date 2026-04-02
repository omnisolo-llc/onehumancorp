CREATE TABLE IF NOT EXISTS embedding_cache (
    content_hash TEXT PRIMARY KEY,
    embedding TEXT NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
