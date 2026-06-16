CREATE TABLE IF NOT EXISTS service_catalog (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    base_price DECIMAL(10, 2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS conversation_context (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    customer_id UUID,
    messages JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quote_drafts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversation_context(id),
    service_id UUID REFERENCES service_catalog(id),
    proposed_price DECIMAL(10, 2),
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE service_catalog ENABLE ROW LEVEL SECURITY;
ALTER TABLE conversation_context ENABLE ROW LEVEL SECURITY;
ALTER TABLE quote_drafts ENABLE ROW LEVEL SECURITY;

CREATE POLICY service_catalog_tenant_policy ON service_catalog
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE POLICY conversation_context_tenant_policy ON conversation_context
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE POLICY quote_drafts_tenant_policy ON quote_drafts
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id')::uuid);
