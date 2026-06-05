CREATE TABLE IF NOT EXISTS ohc_local_reviews (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    review_id TEXT NOT NULL,
    reviewer_name TEXT NOT NULL,
    star_rating INTEGER NOT NULL,
    comment TEXT,
    ai_draft_reply TEXT,
    reply_status TEXT NOT NULL DEFAULT 'PENDING',
    platform TEXT,
    sentiment TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_ohc_local_reviews_tenant_id ON ohc_local_reviews(tenant_id);
CREATE INDEX IF NOT EXISTS idx_ohc_local_reviews_status ON ohc_local_reviews(reply_status);
ALTER TABLE ohc_local_reviews ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_local_reviews ON ohc_local_reviews;
CREATE POLICY tenant_isolation_ohc_local_reviews ON ohc_local_reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
