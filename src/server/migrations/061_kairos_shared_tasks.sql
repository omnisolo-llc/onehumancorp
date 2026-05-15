CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    priority TEXT NOT NULL DEFAULT 'P2',
    agent_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;

ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id TEXT NOT NULL,
    depends_on_task_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id),
    FOREIGN KEY (task_id) REFERENCES shared_tasks(id),
    FOREIGN KEY (depends_on_task_id) REFERENCES shared_tasks(id)
);

ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;

ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
