-- 065_update_rls_for_missing_tenant_id.sql
-- Enforce tenant_id across all relevant tables and remove empty string bypasses.

ALTER TABLE agent_inbox ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE meeting_rooms ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE meeting_transcripts ADD COLUMN IF NOT EXISTS tenant_id TEXT;

-- For agent_inbox and meeting_rooms, organization_id was added in 020_tenant_isolation.sql
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agent_inbox' AND column_name = 'organization_id') THEN
        UPDATE agent_inbox SET tenant_id = organization_id WHERE tenant_id IS NULL;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'meeting_rooms' AND column_name = 'organization_id') THEN
        UPDATE meeting_rooms SET tenant_id = organization_id WHERE tenant_id IS NULL;
    END IF;
END $$;

-- Make sure RLS is enabled
ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;

-- Drop old policies on these three tables safely
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms_t ON meeting_rooms;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox_t ON agent_inbox;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts_t ON meeting_transcripts;

-- Create new robust policies for these three
CREATE POLICY tenant_isolation_meeting_rooms_t ON meeting_rooms USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_agent_inbox_t ON agent_inbox USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_meeting_transcripts_t ON meeting_transcripts USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
