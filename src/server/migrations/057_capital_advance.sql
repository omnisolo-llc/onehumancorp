CREATE TABLE IF NOT EXISTS capital_advances (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    amount_cents BIGINT NOT NULL,
    fee_cents BIGINT NOT NULL,
    total_repayment_cents BIGINT NOT NULL,
    repaid_cents BIGINT DEFAULT 0,
    repayment_percentage DECIMAL NOT NULL,
    status TEXT DEFAULT 'offered', -- 'offered', 'accepted', 'repaying', 'repaid', 'rejected'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS capital_repayments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    advance_id TEXT REFERENCES capital_advances(id) ON DELETE CASCADE,
    source_order_id TEXT, -- ID of the order/booking this repayment was deducted from
    amount_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_advances ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_advances ON capital_advances USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE capital_repayments ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_repayments ON capital_repayments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
