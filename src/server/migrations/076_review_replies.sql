-- Autonomous Reputation & Review Engine
-- Issue #24084

CREATE TABLE IF NOT EXISTS review_replies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    review_id TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Drafted',
    drafted_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_review_replies_tenant ON review_replies(tenant_id);
CREATE INDEX IF NOT EXISTS idx_review_replies_review ON review_replies(review_id);

ALTER TABLE review_replies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_review_replies ON review_replies;
CREATE POLICY tenant_isolation_review_replies ON review_replies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
