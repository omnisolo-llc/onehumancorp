CREATE TABLE IF NOT EXISTS localizable_content (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    resource_id UUID NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    field_name VARCHAR(255) NOT NULL,
    language_code VARCHAR(10) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (tenant_id, resource_id, resource_type, field_name, language_code)
);

ALTER TABLE localizable_content ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_localizable_content
ON localizable_content
USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
