CREATE TABLE IF NOT EXISTS affiliate_links (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    affiliate_code TEXT UNIQUE NOT NULL,
    discount_percentage INTEGER NOT NULL DEFAULT 10,
    commission_percentage INTEGER NOT NULL DEFAULT 10,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_affiliate_links_tenant ON affiliate_links(tenant_id);
CREATE INDEX IF NOT EXISTS idx_affiliate_links_code ON affiliate_links(affiliate_code);
ALTER TABLE affiliate_links ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_links ON affiliate_links;
CREATE POLICY tenant_isolation_affiliate_links
ON affiliate_links
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS affiliate_ledgers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    affiliate_link_id TEXT REFERENCES affiliate_links(id) ON DELETE CASCADE,
    order_id TEXT NOT NULL,
    commission_amount BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_tenant ON affiliate_ledgers(tenant_id);
CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_link ON affiliate_ledgers(affiliate_link_id);
ALTER TABLE affiliate_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_ledgers ON affiliate_ledgers;
CREATE POLICY tenant_isolation_affiliate_ledgers
ON affiliate_ledgers
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS affiliate_payouts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    affiliate_link_id TEXT REFERENCES affiliate_links(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_link ON affiliate_payouts(affiliate_link_id);
ALTER TABLE affiliate_payouts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_payouts ON affiliate_payouts;
CREATE POLICY tenant_isolation_affiliate_payouts
ON affiliate_payouts
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));