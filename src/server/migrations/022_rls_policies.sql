-- Enforce Multi-Tenant Isolation for sensitive data sources
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
CREATE POLICY telemetry_buffer_tenant_isolation ON telemetry_buffer
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

ALTER TABLE business_milestones ENABLE ROW LEVEL SECURITY;
CREATE POLICY business_milestones_tenant_isolation ON business_milestones
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);
