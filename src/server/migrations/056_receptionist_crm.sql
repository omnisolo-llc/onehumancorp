-- Migration 056: Receptionist CRM

-- Create customer_leads table
CREATE TABLE IF NOT EXISTS customer_leads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_name TEXT,
    contact_info TEXT,
    service_needed TEXT,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on customer_leads
ALTER TABLE customer_leads ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_leads ON customer_leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Create service_quotes table
CREATE TABLE IF NOT EXISTS service_quotes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    lead_id TEXT REFERENCES customer_leads(id) ON DELETE CASCADE,
    service_description TEXT NOT NULL,
    estimated_price_cents BIGINT,
    currency TEXT DEFAULT 'USD',
    status TEXT DEFAULT 'draft',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on service_quotes
ALTER TABLE service_quotes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_service_quotes ON service_quotes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
