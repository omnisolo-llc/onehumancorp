-- +goose Up
CREATE TABLE IF NOT EXISTS proposals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
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

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
DROP TABLE IF EXISTS proposal_line_items CASCADE;

DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
DROP TABLE IF EXISTS proposals CASCADE;
