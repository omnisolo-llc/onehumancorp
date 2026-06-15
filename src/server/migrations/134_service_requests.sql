CREATE TABLE IF NOT EXISTS service_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    description TEXT NOT NULL,
    job_type TEXT,
    location TEXT,
    urgency TEXT,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE service_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_service_requests ON service_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE estimates ADD COLUMN IF NOT EXISTS service_request_id TEXT REFERENCES service_requests(id) ON DELETE SET NULL;
ALTER TABLE estimates ADD COLUMN IF NOT EXISTS proposed_time_slots JSONB DEFAULT '[]';
