-- Migration: 003_add_rls_policies.sql
-- Add missing columns to agent_missions and apply ENABLE ROW LEVEL SECURITY policies to missed tables.

-- Add missing columns to agent_missions
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS cloud_mission_id TEXT;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS mission_log TEXT;

-- Enable RLS and apply tenant isolation policies
ALTER TABLE roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_roles ON roles USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_order_items ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true));
