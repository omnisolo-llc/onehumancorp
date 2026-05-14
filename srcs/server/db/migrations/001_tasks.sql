CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    parent_task_id TEXT,
    agent_id TEXT,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'RUNNING', 'COMPLETED', 'FAILED')),
    payload TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT,
    agent_id TEXT,
    task_id TEXT,
    content TEXT,
    embedding vector(3),
    source_type TEXT
);
