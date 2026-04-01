-- 005_sip.sql
-- Swarm Intelligence Protocol tables (migrated from SQLite).

CREATE TABLE IF NOT EXISTS swarm_memory (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS agent_missions (
    id         TEXT PRIMARY KEY,
    status     TEXT NOT NULL,
    payload    TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_missions_status ON agent_missions (status);

CREATE TABLE IF NOT EXISTS agent_status (
    agent_id       TEXT PRIMARY KEY,
    role           TEXT NOT NULL,
    status         TEXT NOT NULL,
    last_heartbeat TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS capability_plugins (
    plugin_id     TEXT PRIMARY KEY,
    name          TEXT NOT NULL,
    version       TEXT NOT NULL,
    manifest_url  TEXT NOT NULL,
    status        TEXT NOT NULL,
    registered_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id        TEXT PRIMARY KEY,
    context          TEXT NOT NULL,
    vector_embedding BYTEA,
    source_plugin    TEXT,
    created_at       TIMESTAMPTZ DEFAULT NOW()
);
