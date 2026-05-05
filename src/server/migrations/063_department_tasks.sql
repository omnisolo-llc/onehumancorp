-- 063_department_tasks.sql

CREATE TABLE IF NOT EXISTS department_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    department TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    status TEXT NOT NULL DEFAULT 'PENDING',
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_department_tasks_polling ON department_tasks (department, status, created_at) WHERE status = 'PENDING';
