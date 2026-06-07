CREATE TABLE IF NOT EXISTS advisory_reports (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'PENDING',
    payload JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_advisory_reports_tenant_id ON advisory_reports(tenant_id);
CREATE INDEX IF NOT EXISTS idx_advisory_reports_status ON advisory_reports(status);

ALTER TABLE advisory_reports ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_advisory_reports ON advisory_reports USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
