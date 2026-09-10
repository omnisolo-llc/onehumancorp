CREATE TABLE IF NOT EXISTS session_capsules (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL,
    customer_id UUID,
    context JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE session_capsules ENABLE ROW LEVEL SECURITY;

CREATE POLICY session_capsules_tenant_isolation_policy ON session_capsules FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
