-- +goose Up
-- Create rag_memories table if it doesn't exist
CREATE TABLE IF NOT EXISTS rag_memories (
    id TEXT PRIMARY KEY,
    context TEXT NOT NULL,
    vector TEXT,
    sync_status VARCHAR(50) DEFAULT 'pending',
    last_sync_at TIMESTAMP NULL
);

-- +goose Down
-- Recreate table without sync columns or omit
