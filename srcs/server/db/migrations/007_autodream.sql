-- 007_autodream.sql
-- AutoDream pgvector and memory consolidation tables.

-- We enable pgvector extension if we are on Postgres.
-- To ensure SQLite parity (since it doesn't have pgvector extension by default or we can't CREATE EXTENSION),
-- we handle standard SQL as much as possible.
-- If the DB provider is Postgres, we will enable the extension.
-- For SQLite, vector operations might be simulated in-memory, so we store vectors as BYTEA/BLOB.
-- But since the prompt specifically mentions "pgvector", we will define it.
-- In `srcs/server/db/database.go`, the migrator replaces BYTEA with BLOB for SQLite.

-- We don't use 'CREATE EXTENSION IF NOT EXISTS vector' directly here because it will fail on SQLite,
-- unless we patch it out in `database.go`. Let's create the tables using standard types first, or patch database.go.

CREATE TABLE IF NOT EXISTS autodream_memory_consolidation (
    id            TEXT PRIMARY KEY,
    agent_id      TEXT NOT NULL,
    original_data TEXT NOT NULL,
    summary       TEXT NOT NULL,
    embedding     vector(1536),
    status        TEXT NOT NULL DEFAULT 'PENDING',
    created_at    TIMESTAMPTZ DEFAULT NOW(),
    resolved_at   TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS autodream_knowledge_graph (
    id            TEXT PRIMARY KEY,
    concept       TEXT NOT NULL,
    description   TEXT NOT NULL,
    embedding     vector(1536),
    created_at    TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_autodream_status ON autodream_memory_consolidation (status);
