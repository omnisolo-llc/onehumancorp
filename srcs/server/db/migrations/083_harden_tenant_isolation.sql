-- +goose Up
-- Enable Row Level Security on sub_agent_queue
ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue
    USING (organization_id = current_setting('app.current_tenant', true));

-- Enable Row Level Security on agent_missions
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions
    USING (organization_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
ALTER TABLE agent_missions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
ALTER TABLE sub_agent_queue DISABLE ROW LEVEL SECURITY;
