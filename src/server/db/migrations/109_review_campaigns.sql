CREATE TABLE IF NOT EXISTS review_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    booking_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'sent', 'completed'
    scheduled_for TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_review_campaigns_tenant ON review_campaigns(tenant_id);
CREATE INDEX IF NOT EXISTS idx_review_campaigns_booking ON review_campaigns(booking_id);

ALTER TABLE review_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_review_campaigns ON review_campaigns;
CREATE POLICY tenant_isolation_review_campaigns ON review_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS review_responses (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    review_id TEXT NOT NULL,
    drafted_content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft', -- 'draft', 'published'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_review_responses_tenant ON review_responses(tenant_id);
CREATE INDEX IF NOT EXISTS idx_review_responses_review ON review_responses(review_id);

ALTER TABLE review_responses ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_review_responses ON review_responses;
CREATE POLICY tenant_isolation_review_responses ON review_responses USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Add source to reviews
ALTER TABLE reviews ADD COLUMN IF NOT EXISTS source TEXT DEFAULT 'sms';
-- Add booking_id to reviews
ALTER TABLE reviews ADD COLUMN IF NOT EXISTS booking_id TEXT;
