-- +goose Up
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    parent_task_id UUID NOT NULL,
    payload JSONB,
    status VARCHAR NOT NULL DEFAULT 'QUEUED',
    worker_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue USING (organization_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS sub_agent_queue;
