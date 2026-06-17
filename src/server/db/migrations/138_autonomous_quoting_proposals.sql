-- +goose Up
CREATE TABLE IF NOT EXISTS proposals (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'DRAFT' CHECK (status IN ('DRAFT', 'PENDING_APPROVAL', 'SENT', 'ACCEPTED', 'REJECTED', 'EXPIRED')),
    scope TEXT NOT NULL,
    total_amount_cents BIGINT,
    required_deposit_cents BIGINT,
    stripe_payment_link TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_proposals_tenant_id ON proposals(tenant_id);

ALTER TABLE proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
CREATE POLICY tenant_isolation_proposals
ON proposals
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS proposal_line_items (
    id UUID PRIMARY KEY,
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_proposal_line_items_proposal_id ON proposal_line_items(proposal_id);

ALTER TABLE proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
CREATE POLICY tenant_isolation_proposal_line_items
ON proposal_line_items
USING (
    proposal_id IN (SELECT id FROM proposals WHERE tenant_id = current_setting('app.current_tenant', true))
)
WITH CHECK (
    proposal_id IN (SELECT id FROM proposals WHERE tenant_id = current_setting('app.current_tenant', true))
);


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
DROP TABLE IF EXISTS proposal_line_items CASCADE;

DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
DROP TABLE IF EXISTS proposals CASCADE;
