-- 057_harden_rls_tool_integrations.sql
-- Drop old vulnerable policies and create new hardened ones without the empty string check.

DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
