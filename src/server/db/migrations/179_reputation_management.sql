-- Migration: Reputation Management Architecture

CREATE TABLE IF NOT EXISTS reputation_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    trigger_event TEXT NOT NULL, -- e.g., 'BookingCompleted', 'FulfillmentCompleted'
    delay_interval INTERVAL NOT NULL DEFAULT '24 hours',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_reputation_campaigns_tenant ON reputation_campaigns(tenant_id);

ALTER TABLE reputation_campaigns ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_reputation_campaigns ON reputation_campaigns
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS feedback_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    campaign_id TEXT REFERENCES reputation_campaigns(id) ON DELETE SET NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    reference_id TEXT NOT NULL, -- The ID of the booking/fulfillment
    reference_type TEXT NOT NULL, -- 'booking', 'fulfillment'
    status TEXT NOT NULL DEFAULT 'scheduled', -- scheduled, sent, replied, intercepted, published
    scheduled_for TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_feedback_requests_tenant ON feedback_requests(tenant_id);
CREATE INDEX IF NOT EXISTS idx_feedback_requests_scheduled_for ON feedback_requests(scheduled_for) WHERE status = 'scheduled';

ALTER TABLE feedback_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_feedback_requests ON feedback_requests
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS customer_reviews (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    feedback_request_id TEXT REFERENCES feedback_requests(id) ON DELETE SET NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    source TEXT NOT NULL DEFAULT 'Internal', -- Internal, Google, Yelp
    status TEXT NOT NULL DEFAULT 'pending', -- pending, published, triaged
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_customer_reviews_tenant ON customer_reviews(tenant_id);

ALTER TABLE customer_reviews ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_reviews ON customer_reviews
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Update missing RLS for existing jobs and queue related logic if any
