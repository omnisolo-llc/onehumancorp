-- 022_autodream_memories.sql

-- We will just alter the table to rename consolidated_at to created_at and change type,
-- or recreate it if we drop it, but dropping might lose data. We can just add the column.
-- Note: SQLite does not support ADD COLUMN IF NOT EXISTS.
-- So we can't reliably do ALTER TABLE IF NOT EXISTS.
-- Let's just create the table. We know it already exists from 007, but IF NOT EXISTS avoids errors.
-- We must NOT use ADD COLUMN IF NOT EXISTS because sqlite doesn't support it.

CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_mission_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
