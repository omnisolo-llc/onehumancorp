-- 017_tenant_isolation.sql
-- Add organization_id to meeting rooms for tenant isolation.

ALTER TABLE meeting_rooms ADD COLUMN organization_id TEXT NOT NULL DEFAULT '';
CREATE INDEX IF NOT EXISTS idx_meeting_rooms_org ON meeting_rooms(organization_id);

ALTER TABLE agent_inbox ADD COLUMN organization_id TEXT NOT NULL DEFAULT '';
CREATE INDEX IF NOT EXISTS idx_agent_inbox_org ON agent_inbox(organization_id);
