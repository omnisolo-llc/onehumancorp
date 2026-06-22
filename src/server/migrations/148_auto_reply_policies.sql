CREATE TABLE IF NOT EXISTS auto_reply_policies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    enabled BOOLEAN NOT NULL DEFAULT true,
    delay_minutes INTEGER NOT NULL DEFAULT 5,
    tone_instructions TEXT DEFAULT '',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id)
);
ALTER TABLE auto_reply_policies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_auto_reply_policies ON auto_reply_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
