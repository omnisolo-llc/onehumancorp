CREATE TABLE IF NOT EXISTS website_drafts (
    id VARCHAR(255) PRIMARY KEY,
    organization_id VARCHAR(255) NOT NULL,
    bio TEXT,
    site_data JSONB,
    status VARCHAR(50) DEFAULT 'DRAFT',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_website_drafts_org_id ON website_drafts(organization_id);

ALTER TABLE website_drafts ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_website_drafts ON website_drafts;
CREATE POLICY tenant_isolation_website_drafts ON website_drafts
    USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
