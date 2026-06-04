-- Autonomous Reputation & Referral Engine
-- Issue #23697

CREATE TABLE IF NOT EXISTS reputation_profiles (
    id TEXT PRIMARY KEY,
    tenant_id TEXT UNIQUE NOT NULL,
    average_rating DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    total_reviews INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_reputation_profiles_tenant ON reputation_profiles(tenant_id);

ALTER TABLE reputation_profiles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reputation_profiles ON reputation_profiles;
CREATE POLICY tenant_isolation_reputation_profiles ON reputation_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS reviews (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    order_id TEXT,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_reviews_tenant ON reviews(tenant_id);

ALTER TABLE reviews ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reviews ON reviews;
CREATE POLICY tenant_isolation_reviews ON reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS referral_codes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    referral_code TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_referral_codes_tenant ON referral_codes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_referral_codes_customer ON referral_codes(tenant_id, customer_id);

ALTER TABLE referral_codes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referral_codes ON referral_codes;
CREATE POLICY tenant_isolation_referral_codes ON referral_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
