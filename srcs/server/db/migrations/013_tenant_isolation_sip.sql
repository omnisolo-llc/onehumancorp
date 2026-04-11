-- 013_tenant_isolation_sip.sql
-- Add organization_id to SIP tables to prevent tenant data leakage in multi-tenant mode

ALTER TABLE swarm_memory ADD COLUMN organization_id TEXT DEFAULT 'system';
ALTER TABLE agent_missions ADD COLUMN organization_id TEXT DEFAULT 'system';
ALTER TABLE agent_status ADD COLUMN organization_id TEXT DEFAULT 'system';
ALTER TABLE capability_plugins ADD COLUMN organization_id TEXT DEFAULT 'system';
ALTER TABLE swarm_memory_embeddings ADD COLUMN organization_id TEXT DEFAULT 'system';
-- ALTER TABLE telemetry_buffer ADD COLUMN organization_id TEXT DEFAULT .system.;

-- Add composite indices for performance and uniqueness where necessary
CREATE INDEX IF NOT EXISTS idx_agent_missions_org_status ON agent_missions(organization_id, status);
CREATE INDEX IF NOT EXISTS idx_agent_status_org ON agent_status(organization_id);
