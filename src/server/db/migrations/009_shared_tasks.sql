CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT,
    parent_task_id TEXT,
    agent_id TEXT,
    assigned_agent_id TEXT,
    title TEXT,
    description TEXT,
    status TEXT DEFAULT 'PENDING',
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
