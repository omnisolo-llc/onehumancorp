-- 055_harden_remaining_rls_policies.sql
-- Drop vulnerable policies that allow empty tenant bypass and recreate them without the bypass.

-- From 050_transcripts_rls.sql
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (
    meeting_id IN (
        SELECT id FROM meeting_rooms
        WHERE organization_id = current_setting('app.current_tenant', true)
           OR current_setting('app.current_tenant', true) = 'system'
    )
);

-- From 054_scout_tool_integrations.sql
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (
    tenant_id = current_setting('app.current_tenant', true)
    OR current_setting('app.current_tenant', true) = 'system'
);
