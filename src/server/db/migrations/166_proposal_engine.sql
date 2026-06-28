-- +goose Up
CREATE TABLE IF NOT EXISTS interactive_proposals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    customer_id UUID,
    status TEXT NOT NULL CHECK (status IN ('Draft', 'Sent', 'Viewed', 'Accepted', 'Paid')),
    total_amount_cents BIGINT NOT NULL DEFAULT 0,
    required_deposit_cents BIGINT NOT NULL DEFAULT 0,
    checkout_url TEXT,
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS interactive_proposal_line_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID NOT NULL REFERENCES interactive_proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS and setup tenant isolation policies
ALTER TABLE interactive_proposals ENABLE ROW LEVEL SECURITY;
ALTER TABLE interactive_proposal_line_items ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
CREATE POLICY tenant_isolation_interactive_proposals ON interactive_proposals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
CREATE POLICY tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items
    USING (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true)))
    WITH CHECK (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true)));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
DROP TABLE IF EXISTS interactive_proposal_line_items CASCADE;
DROP TABLE IF EXISTS interactive_proposals CASCADE;
