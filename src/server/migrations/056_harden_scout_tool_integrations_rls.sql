-- 056_harden_scout_tool_integrations_rls.sql
-- Drop old vulnerable policy and create new hardened one without the empty string check.

DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
