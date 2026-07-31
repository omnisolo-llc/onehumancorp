-- +goose Up
CREATE TABLE IF NOT EXISTS quote_requests (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID,
    status TEXT NOT NULL CHECK (status IN ('NEW', 'TRIAGED', 'ESTIMATING', 'PROPOSAL_DRAFTED', 'CLOSED')),
    source TEXT NOT NULL,
    message TEXT NOT NULL,
    images JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE quote_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_quote_requests ON quote_requests;
DROP TABLE IF EXISTS quote_requests CASCADE;
