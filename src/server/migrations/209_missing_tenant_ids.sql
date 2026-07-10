-- +goose Up
-- Migration 209: Add missing tenant_id columns for RLS

ALTER TABLE IF EXISTS agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT '';
ALTER TABLE IF EXISTS swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT '';
ALTER TABLE IF EXISTS estimate_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT '';

-- Update estimate_line_items tenant_id based on estimates
UPDATE estimate_line_items eli
SET tenant_id = e.tenant_id
FROM estimates e
WHERE eli.estimate_id = e.id;

CREATE INDEX IF NOT EXISTS idx_agent_session_data_tenant_id ON agent_session_data(tenant_id);
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_tenant_id ON swarm_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_estimate_line_items_tenant_id ON estimate_line_items(tenant_id);

-- Apply RLS if not already done
ALTER TABLE IF EXISTS agent_session_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS estimate_line_items ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'agent_session_data' AND policyname = 'tenant_isolation_agent_session_data') THEN
        CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'swarm_tasks' AND policyname = 'tenant_isolation_swarm_tasks') THEN
        CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- Update estimate_line_items policy
    DROP POLICY IF EXISTS tenant_isolation_estimate_line_items ON estimate_line_items;
    CREATE POLICY tenant_isolation_estimate_line_items ON estimate_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
END
$$;
