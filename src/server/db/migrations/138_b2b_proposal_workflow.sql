-- Migration: B2B Proposal & Client Approval Workflows

-- Intake Requests Table
CREATE TABLE IF NOT EXISTS b2b_intake_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    company_name TEXT,
    tax_id TEXT,
    requirements TEXT NOT NULL,
    budget BIGINT,
    timeline TEXT,
    status TEXT NOT NULL DEFAULT 'NEW',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_b2b_intake_requests_tenant ON b2b_intake_requests(tenant_id);

ALTER TABLE b2b_intake_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_b2b_intake_requests ON b2b_intake_requests
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Proposals Table
CREATE TABLE IF NOT EXISTS b2b_proposals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    intake_request_id TEXT NOT NULL REFERENCES b2b_intake_requests(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_b2b_proposals_tenant ON b2b_proposals(tenant_id);

ALTER TABLE b2b_proposals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_b2b_proposals ON b2b_proposals
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- For unauthenticated proposal viewing/approval, allow select/update based on ID matching without tenant context ONLY IF tenant_id = app.current_tenant OR app.current_tenant = '' (empty string for unauth bypass)
DROP POLICY IF EXISTS public_access_b2b_proposals ON b2b_proposals;
CREATE POLICY public_access_b2b_proposals ON b2b_proposals
FOR ALL
USING (current_setting('app.current_tenant', true) = '' OR tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (current_setting('app.current_tenant', true) = '' OR tenant_id::text = current_setting('app.current_tenant', true));


-- Proposal Line Items Table
CREATE TABLE IF NOT EXISTS b2b_proposal_line_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    proposal_id TEXT NOT NULL REFERENCES b2b_proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_b2b_proposal_line_items_tenant ON b2b_proposal_line_items(tenant_id);

ALTER TABLE b2b_proposal_line_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_b2b_proposal_line_items ON b2b_proposal_line_items
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
DROP POLICY IF EXISTS public_access_b2b_proposal_line_items ON b2b_proposal_line_items;
CREATE POLICY public_access_b2b_proposal_line_items ON b2b_proposal_line_items
FOR ALL
USING (current_setting('app.current_tenant', true) = '' OR tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (current_setting('app.current_tenant', true) = '' OR tenant_id::text = current_setting('app.current_tenant', true));


-- Approval Events Table
CREATE TABLE IF NOT EXISTS b2b_approval_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    proposal_id TEXT NOT NULL REFERENCES b2b_proposals(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- e.g., 'viewed', 'approved', 'rejected'
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_b2b_approval_events_tenant ON b2b_approval_events(tenant_id);

ALTER TABLE b2b_approval_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_b2b_approval_events ON b2b_approval_events
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
DROP POLICY IF EXISTS public_access_b2b_approval_events ON b2b_approval_events;
CREATE POLICY public_access_b2b_approval_events ON b2b_approval_events
FOR ALL
USING (current_setting('app.current_tenant', true) = '' OR tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (current_setting('app.current_tenant', true) = '' OR tenant_id::text = current_setting('app.current_tenant', true));
