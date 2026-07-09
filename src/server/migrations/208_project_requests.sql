-- Migration for SQLite embedded compatibility
CREATE TABLE IF NOT EXISTS project_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_name TEXT NOT NULL,
    customer_email TEXT NOT NULL,
    details TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE project_requests ENABLE ROW LEVEL SECURITY;

ALTER TABLE projects ADD COLUMN proposal_id TEXT REFERENCES proposals(id) ON DELETE SET NULL;
