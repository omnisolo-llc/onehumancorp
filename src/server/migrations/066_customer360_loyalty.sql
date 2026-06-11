CREATE TABLE IF NOT EXISTS customer360 (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    mood TEXT,
    preferences TEXT DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_customer360_tenant_customer ON customer360(tenant_id, customer_id);
CREATE TABLE IF NOT EXISTS loyalty_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    points_balance INTEGER DEFAULT 0,
    tier_name TEXT,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, customer_id)
);
CREATE INDEX IF NOT EXISTS idx_loyalty_ledger_tenant_customer ON loyalty_ledger(tenant_id, customer_id);
ALTER TABLE customer360 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer360 ON customer360;
CREATE POLICY tenant_isolation_customer360
ON customer360
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
ALTER TABLE loyalty_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_ledger ON loyalty_ledger;
CREATE POLICY tenant_isolation_loyalty_ledger
ON loyalty_ledger
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));