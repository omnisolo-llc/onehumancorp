-- +goose Up

-- Fix missing RLS policies on tables that have tenant_id

-- 1. bookings
ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 2. consolidated_memory
ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 3. integration_credentials
ALTER TABLE IF EXISTS integration_credentials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_integration_credentials ON integration_credentials;
CREATE POLICY tenant_isolation_integration_credentials ON integration_credentials USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 4. interactive_proposals
ALTER TABLE IF EXISTS interactive_proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
CREATE POLICY tenant_isolation_interactive_proposals ON interactive_proposals USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 5. job_locations
ALTER TABLE IF EXISTS job_locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
CREATE POLICY tenant_isolation_job_locations ON job_locations USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 6. onboarding_state
ALTER TABLE IF EXISTS onboarding_state ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 7. pricing_rules
ALTER TABLE IF EXISTS pricing_rules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_rules ON pricing_rules;
CREATE POLICY tenant_isolation_pricing_rules ON pricing_rules USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 8. seo_discovery_reports
ALTER TABLE IF EXISTS seo_discovery_reports ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_seo_discovery_reports ON seo_discovery_reports;
CREATE POLICY tenant_isolation_seo_discovery_reports ON seo_discovery_reports USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 9. service_routes
ALTER TABLE IF EXISTS service_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
CREATE POLICY tenant_isolation_service_routes ON service_routes USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 10. shifts
ALTER TABLE IF EXISTS shifts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
CREATE POLICY tenant_isolation_shifts ON shifts USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 11. swarm_tasks
ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 12. sync_events
ALTER TABLE IF EXISTS sync_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
CREATE POLICY tenant_isolation_sync_events ON sync_events USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 13. task_dependencies
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 14. tool_integrations
ALTER TABLE IF EXISTS tool_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
-- Revert RLS
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
ALTER TABLE IF EXISTS bookings DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
ALTER TABLE IF EXISTS consolidated_memory DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_integration_credentials ON integration_credentials;
ALTER TABLE IF EXISTS integration_credentials DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
ALTER TABLE IF EXISTS interactive_proposals DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
ALTER TABLE IF EXISTS job_locations DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
ALTER TABLE IF EXISTS onboarding_state DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pricing_rules ON pricing_rules;
ALTER TABLE IF EXISTS pricing_rules DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_seo_discovery_reports ON seo_discovery_reports;
ALTER TABLE IF EXISTS seo_discovery_reports DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
ALTER TABLE IF EXISTS service_routes DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
ALTER TABLE IF EXISTS shifts DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
ALTER TABLE IF EXISTS swarm_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
ALTER TABLE IF EXISTS sync_events DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
ALTER TABLE IF EXISTS task_dependencies DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
ALTER TABLE IF EXISTS tool_integrations DISABLE ROW LEVEL SECURITY;
