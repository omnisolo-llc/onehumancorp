-- +goose Up

CREATE TABLE IF NOT EXISTS b2b_clients (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    company_name TEXT NOT NULL,
    tax_id TEXT,
    primary_contact_name TEXT,
    primary_contact_email TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE b2b_clients ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_b2b_clients ON b2b_clients USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS intake_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    b2b_client_id TEXT REFERENCES b2b_clients(id) ON DELETE SET NULL,
    description TEXT NOT NULL,
    budget_cents BIGINT,
    timeline TEXT,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE intake_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_intake_requests ON intake_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS proposals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    intake_request_id TEXT REFERENCES intake_requests(id) ON DELETE SET NULL,
    b2b_client_id TEXT REFERENCES b2b_clients(id) ON DELETE SET NULL,
    status TEXT DEFAULT 'DRAFT',
    total_amount_cents BIGINT DEFAULT 0,
    required_deposit_cents BIGINT DEFAULT 0,
    checkout_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE proposals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_proposals ON proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS proposal_line_items (
    id TEXT PRIMARY KEY,
    proposal_id TEXT NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE proposal_line_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items USING (
    proposal_id IN (SELECT id FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true))
) WITH CHECK (
    proposal_id IN (SELECT id FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true))
);

CREATE TABLE IF NOT EXISTS approval_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    proposal_id TEXT REFERENCES proposals(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE approval_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_approval_events ON approval_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_approval_events ON approval_events;
DROP TABLE IF EXISTS approval_events CASCADE;

DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
DROP TABLE IF EXISTS proposal_line_items CASCADE;

DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
DROP TABLE IF EXISTS proposals CASCADE;

DROP POLICY IF EXISTS tenant_isolation_intake_requests ON intake_requests;
DROP TABLE IF EXISTS intake_requests CASCADE;

DROP POLICY IF EXISTS tenant_isolation_b2b_clients ON b2b_clients;
DROP TABLE IF EXISTS b2b_clients CASCADE;
