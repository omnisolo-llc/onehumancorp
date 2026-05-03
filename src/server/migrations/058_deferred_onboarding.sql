CREATE TABLE IF NOT EXISTS storefront_drafts (
    id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    business_type VARCHAR(255) NOT NULL,
    instagram_handle VARCHAR(255),
    company_name VARCHAR(255) NOT NULL,
    company_description TEXT NOT NULL,
    approved BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE storefront_drafts ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_storefront_drafts ON storefront_drafts;
CREATE POLICY tenant_isolation_storefront_drafts ON storefront_drafts USING (
    tenant_id = current_setting('app.current_tenant', true)
);
