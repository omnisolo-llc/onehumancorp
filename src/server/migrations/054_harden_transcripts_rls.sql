-- 054_harden_transcripts_rls.sql
-- Drop old vulnerable policy and create a new hardened one without the empty string check.

DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (
    meeting_id IN (
        SELECT id FROM meeting_rooms
        WHERE organization_id = current_setting('app.current_tenant', true)
           OR current_setting('app.current_tenant', true) = 'system'
    )
);

-- Similarly verify if referrals was done properly in 053.
-- Referrals is completely fine in 053 as:
-- CREATE POLICY referrals_isolation_policy ON referrals USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
