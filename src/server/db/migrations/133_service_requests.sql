-- +goose Up
CREATE TABLE IF NOT EXISTS service_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    description TEXT,
    urgency TEXT,
    location TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_service_requests_tenant ON service_requests(tenant_id);

ALTER TABLE service_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_service_requests ON service_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_service_requests ON service_requests;
DROP TABLE IF EXISTS service_requests CASCADE;
