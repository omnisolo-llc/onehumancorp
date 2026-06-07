CREATE TABLE IF NOT EXISTS customer_inquiries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    customer_name TEXT,
    customer_email TEXT,
    customer_phone TEXT,
    description TEXT NOT NULL,
    status TEXT DEFAULT 'New', -- New, InProgress, ProposalSent, Closed
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE customer_inquiries ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_inquiries ON customer_inquiries
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS proposals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inquiry_id TEXT REFERENCES customer_inquiries(id) ON DELETE SET NULL,
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    status TEXT DEFAULT 'Draft', -- Draft, Sent, Accepted, Rejected, Expired
    total_amount_cents BIGINT DEFAULT 0,
    deposit_percentage INTEGER DEFAULT 0,
    deposit_amount_cents BIGINT DEFAULT 0,
    payment_intent_id TEXT, -- Stripe payment intent for the deposit
    payment_link_url TEXT, -- Stripe payment link for the customer
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE proposals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_proposals ON proposals
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS proposal_line_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    proposal_id TEXT NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity INTEGER DEFAULT 1,
    unit_price_cents BIGINT DEFAULT 0,
    total_price_cents BIGINT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE proposal_line_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
