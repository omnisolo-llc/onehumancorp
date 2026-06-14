CREATE TABLE IF NOT EXISTS quote_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE quote_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS estimates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    quote_request_id TEXT REFERENCES quote_requests(id) ON DELETE SET NULL,
    customer_id TEXT NOT NULL,
    status TEXT DEFAULT 'DRAFT',
    total_amount_cents BIGINT DEFAULT 0,
    required_deposit_cents BIGINT DEFAULT 0,
    checkout_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE estimates ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_estimates ON estimates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS estimate_line_items (
    id TEXT PRIMARY KEY,
    estimate_id TEXT NOT NULL REFERENCES estimates(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE estimate_line_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_estimate_line_items ON estimate_line_items USING (
    estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true))
) WITH CHECK (
    estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true))
);
