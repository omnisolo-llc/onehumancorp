CREATE TABLE IF NOT EXISTS integration_credentials (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    integration_id TEXT NOT NULL,
    bot_token TEXT,
    api_token TEXT,
    from_phone TEXT,
    chat_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE IF EXISTS integration_credentials ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'integration_credentials'
          AND policyname = 'tenant_isolation_integration_credentials'
    ) THEN
        CREATE POLICY tenant_isolation_integration_credentials ON integration_credentials
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;
