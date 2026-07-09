CREATE TABLE IF NOT EXISTS project_requests (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'NEW',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE project_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_project_requests ON project_requests
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
