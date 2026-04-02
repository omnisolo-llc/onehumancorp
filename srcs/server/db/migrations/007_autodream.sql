-- 007_autodream.sql
-- AutoDream: Vector Embedding and Truth Injection for Agent Memories

-- We use pgvector for similarity searches in Postgres.
-- The Go `db.Provider` abstraction converts this to TEXT for SQLite fallback if needed.

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS agent_session_data (
    session_id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    context_data TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_accessed TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_agent_session_accessed ON agent_session_data(last_accessed);

-- Extend swarm_memory_embeddings if necessary or manage migrations safely
-- but as instructed we will use vector logic.
-- In Postgres we will change BYTEA to VECTOR.
-- However, we must modify the 005_sip.sql to use VECTOR, but since we can't alter an existing deployed table easily without ALTER,
-- we'll add a new table for high-dimensional semantic truth injection.

CREATE TABLE IF NOT EXISTS swarm_truth_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT NOT NULL,
    embedding VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS memory_conflicts (
    conflict_id TEXT PRIMARY KEY,
    memory_id_1 TEXT NOT NULL,
    memory_id_2 TEXT NOT NULL,
    resolution_status TEXT NOT NULL DEFAULT 'PENDING',
    resolved_memory_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
