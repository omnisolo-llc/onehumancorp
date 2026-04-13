CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_task_id TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    worker_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status ON sub_agent_queue(status);
