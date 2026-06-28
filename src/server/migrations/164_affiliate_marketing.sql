CREATE TABLE IF NOT EXISTS affiliate_links (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    code TEXT UNIQUE NOT NULL,
    discount_percentage INTEGER DEFAULT 20,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_affiliate_links_tenant ON affiliate_links(tenant_id);
ALTER TABLE affiliate_links ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_links ON affiliate_links;
CREATE POLICY tenant_isolation_affiliate_links ON affiliate_links USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS affiliate_ledgers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    total_earnings NUMERIC(10, 2) DEFAULT 0.00,
    available_balance NUMERIC(10, 2) DEFAULT 0.00,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, customer_id)
);
CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_tenant ON affiliate_ledgers(tenant_id);
ALTER TABLE affiliate_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_ledgers ON affiliate_ledgers;
CREATE POLICY tenant_isolation_affiliate_ledgers ON affiliate_ledgers USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS affiliate_payouts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    ledger_id TEXT NOT NULL REFERENCES affiliate_ledgers(id) ON DELETE CASCADE,
    amount NUMERIC(10, 2) NOT NULL,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
ALTER TABLE affiliate_payouts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_payouts ON affiliate_payouts;
CREATE POLICY tenant_isolation_affiliate_payouts ON affiliate_payouts USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
