CREATE TABLE IF NOT EXISTS website_configurations (
    organization_id TEXT PRIMARY KEY,
    template TEXT NOT NULL,
    primary_color TEXT NOT NULL,
    domain_choice TEXT NOT NULL,
    url TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE website_configurations ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_website_config ON website_configurations
    FOR ALL
    USING (organization_id = current_setting('app.current_tenant', true));
