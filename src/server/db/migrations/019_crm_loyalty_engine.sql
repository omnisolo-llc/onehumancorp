-- Autonomous Customer Lifecycle & Loyalty Engine

CREATE TABLE IF NOT EXISTS ohc_customer360 (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    mood TEXT NOT NULL DEFAULT 'Neutral',
    preferences JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_customer360_tenant ON ohc_customer360(tenant_id);

ALTER TABLE ohc_customer360 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_customer360 ON ohc_customer360;
CREATE POLICY tenant_isolation_ohc_customer360
ON ohc_customer360
USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_interaction_timeline (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES ohc_customer360(id) ON DELETE CASCADE,
    source TEXT NOT NULL,
    sentiment TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_interaction_timeline_tenant ON ohc_interaction_timeline(tenant_id);
CREATE INDEX IF NOT EXISTS idx_ohc_interaction_timeline_customer ON ohc_interaction_timeline(customer_id);

ALTER TABLE ohc_interaction_timeline ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_interaction_timeline ON ohc_interaction_timeline;
CREATE POLICY tenant_isolation_ohc_interaction_timeline
ON ohc_interaction_timeline
USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_loyalty_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES ohc_customer360(id) ON DELETE CASCADE,
    points_balance INTEGER NOT NULL DEFAULT 0,
    tier_name TEXT NOT NULL DEFAULT 'Standard',
    last_updated TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_loyalty_ledger_tenant ON ohc_loyalty_ledger(tenant_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_ohc_loyalty_ledger_customer ON ohc_loyalty_ledger(customer_id);

ALTER TABLE ohc_loyalty_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_loyalty_ledger ON ohc_loyalty_ledger;
CREATE POLICY tenant_isolation_ohc_loyalty_ledger
ON ohc_loyalty_ledger
USING (tenant_id = current_setting('app.current_tenant', true));
