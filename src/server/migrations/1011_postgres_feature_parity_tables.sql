-- PostgreSQL parity for feature tables that previously existed only in the
-- legacy SQLite migration tree.

CREATE TABLE IF NOT EXISTS tool_integrations (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    api_url TEXT,
    integration_code TEXT,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id)
);

CREATE TABLE IF NOT EXISTS proposals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    total_amount_cents BIGINT NOT NULL DEFAULT 0,
    required_deposit_cents BIGINT NOT NULL DEFAULT 0,
    checkout_url TEXT,
    project_scope TEXT,
    milestones JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS proposal_line_items (
    id TEXT PRIMARY KEY,
    proposal_id TEXT NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS fulfillment_batches (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_plan_id TEXT NOT NULL,
    fulfillment_date DATE NOT NULL,
    subscriber_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subscription_plan_id, tenant_id)
        REFERENCES subscription_plans(id, tenant_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tool_integrations_tenant
    ON tool_integrations (tenant_id);
CREATE INDEX IF NOT EXISTS idx_proposals_tenant
    ON proposals (tenant_id);
CREATE INDEX IF NOT EXISTS idx_proposal_line_items_proposal
    ON proposal_line_items (proposal_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_tenant_plan
    ON fulfillment_batches (tenant_id, subscription_plan_id);

ALTER TABLE tool_integrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE tool_integrations FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE proposals ENABLE ROW LEVEL SECURITY;
ALTER TABLE proposals FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
CREATE POLICY tenant_isolation_proposals ON proposals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE proposal_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE proposal_line_items FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items
    USING (proposal_id IN (
        SELECT id FROM proposals
        WHERE tenant_id = current_setting('app.current_tenant', true)
    ))
    WITH CHECK (proposal_id IN (
        SELECT id FROM proposals
        WHERE tenant_id = current_setting('app.current_tenant', true)
    ));

ALTER TABLE fulfillment_batches ENABLE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_batches FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches ON fulfillment_batches
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

GRANT ALL PRIVILEGES ON
    tool_integrations, proposals, proposal_line_items, fulfillment_batches
TO ohc_bypassrls;
