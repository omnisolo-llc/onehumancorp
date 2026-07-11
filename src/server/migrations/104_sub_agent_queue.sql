-- Migration 084: Create the legacy sub-agent queue table used by onboarding and hybrid sync paths.

CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    parent_task_id VARCHAR,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    agent_role VARCHAR(255),
    status VARCHAR NOT NULL DEFAULT 'QUEUED',
    worker_id VARCHAR,
    scheduled_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_tenant_status
    ON sub_agent_queue(tenant_id, status, scheduled_at);

CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status_schedule
    ON sub_agent_queue(status, scheduled_at, created_at);

ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
CREATE POLICY tenant_isolation_sub_agent_queue
    ON sub_agent_queue
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
