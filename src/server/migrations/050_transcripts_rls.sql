-- 050_transcripts_rls.sql
-- Enable Row Level Security and corresponding policies on meeting_transcripts table.

ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (
    meeting_id IN (
        SELECT id FROM meeting_rooms
        WHERE organization_id = current_setting('app.current_tenant', true)
           OR current_setting('app.current_tenant', true) = 'system'
           OR current_setting('app.current_tenant', true) = ''
    )
);
