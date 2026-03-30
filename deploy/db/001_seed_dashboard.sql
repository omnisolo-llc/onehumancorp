-- Seed the OHC Central Database with initial enterprise data
-- This fulfills the requirement for observability of the hybrid handoff/virtual meeting rooms

CREATE TABLE IF NOT EXISTS swarm_memory (
    key TEXT PRIMARY KEY,
    value TEXT,
    updated_at DATETIME
);

CREATE TABLE IF NOT EXISTS agent_status (
    agent_id TEXT PRIMARY KEY,
    role TEXT,
    status TEXT,
    last_heartbeat DATETIME
);

CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    role TEXT,
    task TEXT,
    status TEXT,
    assigned_to TEXT,
    created_at DATETIME,
    updated_at DATETIME
);

CREATE TABLE IF NOT EXISTS capability_plugins (
    plugin_id TEXT PRIMARY KEY,
    name TEXT,
    version TEXT,
    manifest_url TEXT,
    status TEXT,
    registered_at DATETIME
);

CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT,
    vector_embedding BLOB,
    source_plugin TEXT,
    created_at DATETIME
);

-- Seed Agents
INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES
('agent-director-001', 'Director', 'Running', datetime('now')),
('agent-pm-001', 'Product Manager', 'Running', datetime('now')),
('agent-swe-001', 'Software Engineer', 'Running', datetime('now'))
ON CONFLICT(agent_id) DO UPDATE SET status=excluded.status, last_heartbeat=excluded.last_heartbeat;

-- Seed Active Virtual Meetings into Swarm Memory (JSON)
INSERT INTO swarm_memory (key, value, updated_at) VALUES
('active_meetings', '[{"id": "Room Alpha", "status": "Director reviewing product spec with PM"}, {"id": "Room Beta", "status": "SWEs resolving merge conflict"}]', datetime('now'))
ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=excluded.updated_at;
