CREATE TABLE IF NOT EXISTS whatsapp_connections (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    phone_number_id TEXT NOT NULL,
    waba_id TEXT NOT NULL,
    access_token TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(phone_number_id)
);

ALTER TABLE IF EXISTS whatsapp_connections ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'whatsapp_connections'
          AND policyname = 'tenant_isolation_whatsapp_connections'
    ) THEN
        CREATE POLICY tenant_isolation_whatsapp_connections ON whatsapp_connections
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;