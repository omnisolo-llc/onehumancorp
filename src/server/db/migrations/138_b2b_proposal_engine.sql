-- +goose Up
CREATE TABLE IF NOT EXISTS intake_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    client_name TEXT NOT NULL,
    client_email TEXT NOT NULL,
    client_company TEXT,
    description TEXT NOT NULL,
    budget_cents BIGINT,
    status TEXT NOT NULL DEFAULT 'NEW' CHECK (status IN ('NEW', 'PROPOSAL_DRAFTED', 'PROPOSAL_SENT', 'APPROVED', 'REJECTED')),
    quote_id TEXT REFERENCES quotes(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_intake_requests_tenant ON intake_requests(tenant_id);

ALTER TABLE intake_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_intake_requests ON intake_requests USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_intake_requests ON intake_requests;
DROP TABLE IF EXISTS intake_requests CASCADE;
