-- Missing RLS for booking_slots
ALTER TABLE IF EXISTS booking_slots ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_slots ON booking_slots;
CREATE POLICY tenant_isolation_booking_slots ON booking_slots USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Missing RLS for builder_brand_toolboxes
ALTER TABLE IF EXISTS builder_brand_toolboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_brand_toolboxes ON builder_brand_toolboxes;
CREATE POLICY tenant_isolation_builder_brand_toolboxes ON builder_brand_toolboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Missing RLS for customer360
ALTER TABLE IF EXISTS customer360 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer360 ON customer360;
CREATE POLICY tenant_isolation_customer360 ON customer360 USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
