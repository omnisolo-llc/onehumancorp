CREATE TABLE usage_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    task_id TEXT,
    attempt_id TEXT,
    provider TEXT NOT NULL,
    model TEXT,
    payer TEXT NOT NULL,
    rate_revision TEXT,
    cpu_seconds_active DOUBLE PRECISION NOT NULL DEFAULT 0,
    memory_time_provisioned DOUBLE PRECISION NOT NULL DEFAULT 0,
    sandbox_time_reserved DOUBLE PRECISION NOT NULL DEFAULT 0,
    input_tokens BIGINT NOT NULL DEFAULT 0,
    output_tokens BIGINT NOT NULL DEFAULT 0,
    tool_usage_count BIGINT NOT NULL DEFAULT 0,
    idempotency_key TEXT UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

CREATE INDEX idx_usage_events_tenant_payer ON usage_events(tenant_id, payer);
CREATE INDEX idx_usage_events_task ON usage_events(task_id, attempt_id);
