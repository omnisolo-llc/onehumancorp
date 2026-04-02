CREATE TABLE IF NOT EXISTS embedding_cache (
    content_hash TEXT PRIMARY KEY,
    embedding TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
