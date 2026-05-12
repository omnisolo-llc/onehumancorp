-- 007_autodream.sql
-- AutoDream: Vector Embedding and Truth Injection for Agent Memories

CREATE TABLE IF NOT EXISTS agent_session_data (
    session_id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    context_data TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_accessed TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_agent_session_accessed ON agent_session_data(last_accessed);

CREATE TABLE IF NOT EXISTS swarm_truth_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT NOT NULL,
    embedding VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS memory_conflicts (
    conflict_id TEXT PRIMARY KEY,
    memory_id_1 TEXT NOT NULL,
    memory_id_2 TEXT NOT NULL,
    resolution_status TEXT NOT NULL DEFAULT 'PENDING',
    resolved_memory_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
