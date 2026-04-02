-- 007_autodream.sql
-- Add pgvector support for semantic memory in AutoDream if supported

CREATE EXTENSION IF NOT EXISTS vector;

-- Swarm memory embeddings uses BYTEA in SQLite compatibility mode.
-- For true Postgres with pgvector, we should cast/add a new column or just alter if using pure Postgres.
-- Since the codebase allows generic provider translations, we can add a vector column specifically for Postgres context.

ALTER TABLE swarm_memory_embeddings
ADD COLUMN IF NOT EXISTS pg_vector vector(1536);

-- Distributed Lock Table for background workers
CREATE TABLE IF NOT EXISTS autodream_locks (
    name TEXT PRIMARY KEY,
    expires_at TIMESTAMPTZ NOT NULL
);
