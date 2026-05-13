CREATE TABLE IF NOT EXISTS swarm_tasks (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL DEFAULT 'PENDING',
    dependencies TEXT NOT NULL DEFAULT '[]',
    assigned_agent_id TEXT,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    payload TEXT,
    title TEXT,
    description TEXT,
    priority TEXT
);

CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL DEFAULT 'PENDING',
    dependencies TEXT NOT NULL DEFAULT '[]',
    assigned_agent_id TEXT,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    payload TEXT,
    title TEXT,
    description TEXT,
    priority TEXT
);
