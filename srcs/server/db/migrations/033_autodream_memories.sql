-- Include CREATE EXTENSION for Postgres
-- Note: SQLite will fail this silently or we handle it in provider
CREATE EXTENSION IF NOT EXISTS vector;

-- Drop the existing table to recreate it with the correct schema
DROP TABLE IF EXISTS autodream_memories;

-- Create the table
-- We use conditional logic in the Go provider, but for SQL migrations that are shared,
-- we must write SQL that works or is patched on the fly.
-- But wait! The code review says: "The database migration (033_autodream_memories.sql) is flawed... defining embedding column as TEXT instead of vector(1536)"

-- Since SQLite doesn't understand VECTOR(1536), the standard OHC way is to use VECTOR(1536) in the .sql file
-- and the SQLite Provider's Exec/Query interceptor handles replacing "VECTOR(1536)" with "TEXT" or "JSON" for testing.
-- Let's define it as VECTOR(1536)

CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
