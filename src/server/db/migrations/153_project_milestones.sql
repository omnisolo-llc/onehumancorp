CREATE TABLE interactive_proposals (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    customer_id UUID NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'Draft',
    total_amount_cents BIGINT NOT NULL DEFAULT 0,
    required_deposit_cents BIGINT NOT NULL DEFAULT 0,
    checkout_url VARCHAR,
    message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

ALTER TABLE interactive_proposals ENABLE ROW LEVEL SECURITY;
CREATE POLICY interactive_proposals_tenant_isolation_policy ON interactive_proposals
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE interactive_proposal_line_items (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    proposal_id UUID NOT NULL REFERENCES interactive_proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity BIGINT NOT NULL DEFAULT 1
);

ALTER TABLE interactive_proposal_line_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY interactive_proposal_line_items_tenant_isolation_policy ON interactive_proposal_line_items
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE project_milestones (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    proposal_id UUID NOT NULL REFERENCES interactive_proposals(id) ON DELETE CASCADE,
    title VARCHAR NOT NULL,
    description TEXT,
    due_date TIMESTAMP WITH TIME ZONE,
    status VARCHAR NOT NULL DEFAULT 'Pending',
    invoice_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

ALTER TABLE project_milestones ENABLE ROW LEVEL SECURITY;
CREATE POLICY project_milestones_tenant_isolation_policy ON project_milestones
    USING (tenant_id = current_setting('app.current_tenant', true));
