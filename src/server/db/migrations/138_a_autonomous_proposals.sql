-- +goose Up
-- Migration: Autonomous Proposals & Dynamic Proposal Engine

CREATE TABLE IF NOT EXISTS proposals (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID,
    status TEXT NOT NULL CHECK (status IN ('DRAFT', 'SENT', 'ACCEPTED', 'REJECTED')),
    total_amount_cents BIGINT NOT NULL DEFAULT 0,
    required_deposit_cents BIGINT NOT NULL DEFAULT 0,
    valid_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS proposal_line_items (
    id UUID PRIMARY KEY,
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS and setup tenant isolation policies
ALTER TABLE proposals ENABLE ROW LEVEL SECURITY;
ALTER TABLE proposal_line_items ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
CREATE POLICY tenant_isolation_proposals ON proposals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items
    USING (proposal_id IN (SELECT id FROM proposals WHERE tenant_id = current_setting('app.current_tenant', true)))
    WITH CHECK (proposal_id IN (SELECT id FROM proposals WHERE tenant_id = current_setting('app.current_tenant', true)));

-- +goose Down
DROP TABLE IF EXISTS proposal_line_items CASCADE;
DROP TABLE IF EXISTS proposals CASCADE;
