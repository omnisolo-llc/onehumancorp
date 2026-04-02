CREATE TABLE IF NOT EXISTS reason_cache (
    prompt_hash TEXT PRIMARY KEY,
    response TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);