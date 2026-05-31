CREATE TABLE IF NOT EXISTS action_tokens (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    approval_request_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

ALTER TABLE action_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_action_tokens ON action_tokens USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
