-- 020_tenant_isolation.sql
-- Add tenant_id to meeting rooms for tenant isolation.

ALTER TABLE meeting_rooms ADD COLUMN tenant_id TEXT NOT NULL DEFAULT '';
CREATE INDEX idx_meeting_rooms_org ON meeting_rooms(tenant_id);

ALTER TABLE agent_inbox ADD COLUMN tenant_id TEXT NOT NULL DEFAULT '';
CREATE INDEX idx_agent_inbox_org ON agent_inbox(tenant_id);
