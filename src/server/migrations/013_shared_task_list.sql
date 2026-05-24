CREATE TABLE IF NOT EXISTS shared_task_list (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    epic_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT NOT NULL DEFAULT 'P2',
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
