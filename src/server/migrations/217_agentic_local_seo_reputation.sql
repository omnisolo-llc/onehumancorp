-- Migration: 217_agentic_local_seo_reputation.sql

CREATE TABLE IF NOT EXISTS location_syncs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    platform VARCHAR(255) NOT NULL, -- e.g., 'google_business', 'yelp'
    platform_location_id VARCHAR(255),
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(255),
    state VARCHAR(255),
    postal_code VARCHAR(255),
    country VARCHAR(255),
    business_hours JSONB,
    holiday_hours JSONB,
    sync_status VARCHAR(50) DEFAULT 'pending',
    last_synced_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT location_syncs_tenant_platform_key UNIQUE (tenant_id, platform)
);

CREATE TABLE IF NOT EXISTS reputation_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    platform VARCHAR(255) NOT NULL, -- e.g., 'google_business', 'yelp'
    platform_review_id VARCHAR(255) NOT NULL,
    reviewer_name VARCHAR(255),
    reviewer_customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    rating INTEGER NOT NULL,
    review_text TEXT,
    review_date TIMESTAMPTZ,
    ai_drafted_response TEXT,
    owner_approved_response TEXT,
    response_status VARCHAR(50) DEFAULT 'pending', -- 'pending', 'drafted', 'approved', 'posted', 'flagged'
    posted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT reputation_reviews_tenant_platform_review_key UNIQUE (tenant_id, platform, platform_review_id)
);

-- RLS Policies

ALTER TABLE location_syncs ENABLE ROW LEVEL SECURITY;

CREATE POLICY location_syncs_tenant_isolation_policy ON location_syncs
    USING (tenant_id = current_setting('app.current_tenant')::uuid);

ALTER TABLE reputation_reviews ENABLE ROW LEVEL SECURITY;

CREATE POLICY reputation_reviews_tenant_isolation_policy ON reputation_reviews
    USING (tenant_id = current_setting('app.current_tenant')::uuid);
