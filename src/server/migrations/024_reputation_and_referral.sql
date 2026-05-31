-- Migration 024: Autonomous Reputation and Referral Engine

CREATE TABLE IF NOT EXISTS reputation_profiles (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    aggregate_rating FLOAT DEFAULT 0.0,
    review_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS reviews (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    transaction_id TEXT,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    verified_purchase BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS universal_wallet_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    credit_amount FLOAT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on new tables
ALTER TABLE reputation_profiles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_reputation_profiles ON reputation_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE reviews ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_reviews ON reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE universal_wallet_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_universal_wallet_ledger ON universal_wallet_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
