-- Add indexes for performance optimization of mission queue management
CREATE INDEX IF NOT EXISTS idx_agent_missions_status_tenant ON agent_missions (status, tenant_id);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status_tenant ON sub_agent_queue (status, tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_missions_updated_at ON agent_missions (updated_at);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_updated_at ON sub_agent_queue (updated_at);
