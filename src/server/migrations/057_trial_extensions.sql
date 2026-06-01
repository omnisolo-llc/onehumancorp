CREATE TABLE IF NOT EXISTS trial_extensions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    task_name VARCHAR(255) NOT NULL,
    days_added INT NOT NULL,
    created_at_unix BIGINT DEFAULT 0
);
ALTER TABLE trial_extensions ENABLE ROW LEVEL SECURITY;
