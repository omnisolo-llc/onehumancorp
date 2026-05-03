-- 055_fix_rls_policies.sql
-- Drop the vulnerable RLS policies that allow bypass via empty tenant context
-- and recreate them securely without the empty string check.

-- 1. Hardening tool_integrations (from 054_scout_tool_integrations.sql)
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (
    tenant_id = current_setting('app.current_tenant', true)
    OR current_setting('app.current_tenant', true) = 'system'
);

-- 2. Hardening meeting_transcripts (from 050_transcripts_rls.sql)
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (
    meeting_id IN (
        SELECT id FROM meeting_rooms
        WHERE organization_id = current_setting('app.current_tenant', true)
           OR current_setting('app.current_tenant', true) = 'system'
    )
);
