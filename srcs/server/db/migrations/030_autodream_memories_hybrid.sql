-- Standalone mode (SQLite) requires graceful degradation since it does not support
-- pgvector or certain CREATE EXTENSION syntax natively in the schema setup via text.
-- We ensure the schema exists and conditionally use JSON or TEXT blobs.

-- For PostgreSQL:
-- CREATE EXTENSION IF NOT EXISTS vector;
-- Note: the application code running migrations handles conditional pgvector creation.
-- In pure SQL, creating vector in SQLite fails, so we omit 'CREATE EXTENSION' here
-- if we are targeting pure SQLite compatibility, OR we handle it in code.

-- Because KAIROS AutoDream design doc says:
-- "Provide a SQLite equivalent (e.g., storing embedding as a JSON text blob) for Standalone degradation."
-- We will adjust the previously created tables.

-- In 024 and 029 we created autodream_memories but we need to ensure it has the correct schema.
-- Rather than drop table, we'll create the final shape if it doesn't exist, which it might not
-- if we're running tests on a clean db.

CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    -- Embedding will be vector(1536) in PG, TEXT/JSON in SQLite. We can just use TEXT here for generic compat,
    -- or we alter it dynamically. Since migrations run on both, using TEXT is safer for SQLite,
    -- and we can cast in Postgres or rely on the DB driver. Wait, in 024 it was:
    -- embedding VECTOR(1536),
    -- Let's define it generally:
    embedding TEXT,
    source_type TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
