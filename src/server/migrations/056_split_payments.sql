-- Migration 056: Autonomous Split Payments & Commission Engine

CREATE TABLE IF NOT EXISTS split_payment_rules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    partner_id TEXT NOT NULL,
    partner_phone_or_email TEXT NOT NULL,
    split_type TEXT NOT NULL, -- 'percentage' or 'flat'
    split_value DECIMAL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE split_payment_rules ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_split_payment_rules ON split_payment_rules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
