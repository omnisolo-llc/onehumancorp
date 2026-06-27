-- +goose Up

CREATE TABLE IF NOT EXISTS reputation_reviews (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    order_id TEXT, -- nullable, might link to booking instead
    booking_id TEXT,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    feedback_text TEXT,
    sentiment TEXT CHECK (sentiment IN ('positive', 'negative', 'neutral')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'pulse_sent', 'replied', 'public_prompted', 'owner_escalated')),
    pulse_sent_at TIMESTAMPTZ,
    escalated_task_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_reputation_reviews_tenant_id ON reputation_reviews(tenant_id);
CREATE INDEX IF NOT EXISTS idx_reputation_reviews_customer_id ON reputation_reviews(customer_id);

ALTER TABLE reputation_reviews ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reputation_reviews ON reputation_reviews;
CREATE POLICY tenant_isolation_reputation_reviews
ON reputation_reviews
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS reputation_settings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT UNIQUE NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    auto_request_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    delay_hours INTEGER NOT NULL DEFAULT 2,
    google_review_link TEXT,
    yelp_review_link TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE reputation_settings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reputation_settings ON reputation_settings;
CREATE POLICY tenant_isolation_reputation_settings
ON reputation_settings
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_reputation_reviews ON reputation_reviews;
DROP TABLE IF EXISTS reputation_reviews CASCADE;

DROP POLICY IF EXISTS tenant_isolation_reputation_settings ON reputation_settings;
DROP TABLE IF EXISTS reputation_settings CASCADE;
