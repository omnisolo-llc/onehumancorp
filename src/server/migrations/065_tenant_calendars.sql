-- 065_tenant_calendars.sql
-- Implements the tenant_calendars table to support the service scheduling part of the hybrid catalog.

CREATE TABLE IF NOT EXISTS tenant_calendars (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES tenants(tenant_id) ON DELETE CASCADE,
    product_id TEXT NOT NULL,
    duration_minutes INT NOT NULL,
    availability_schedule JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tenant_calendars_tenant ON tenant_calendars (tenant_id);
CREATE INDEX idx_tenant_calendars_product ON tenant_calendars (product_id);

ALTER TABLE tenant_calendars ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_calendars ON tenant_calendars
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
