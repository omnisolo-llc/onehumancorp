-- +goose Up
CREATE TABLE IF NOT EXISTS project_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_name TEXT NOT NULL,
    customer_email TEXT NOT NULL,
    details TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE project_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_project_requests ON project_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE projects ADD COLUMN IF NOT EXISTS proposal_id TEXT REFERENCES proposals(id) ON DELETE SET NULL;

-- +goose Down
ALTER TABLE projects DROP COLUMN IF EXISTS proposal_id;
DROP POLICY IF EXISTS tenant_isolation_project_requests ON project_requests;
DROP TABLE IF EXISTS project_requests CASCADE;
