CREATE TABLE IF NOT EXISTS wallet_passes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    pass_type TEXT NOT NULL, -- e.g., 'apple', 'google'
    pass_identifier TEXT NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

ALTER TABLE wallet_passes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_wallet_passes ON wallet_passes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
