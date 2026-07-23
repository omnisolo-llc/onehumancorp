-- Add missing RLS to api_keys and user_usage_logs
ALTER TABLE IF EXISTS api_keys ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS user_usage_logs ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies WHERE tablename = 'api_keys' AND policyname = 'tenant_isolation_api_keys'
    ) THEN
        CREATE POLICY tenant_isolation_api_keys ON api_keys
            USING (organization_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_policies WHERE tablename = 'user_usage_logs' AND policyname = 'tenant_isolation_user_usage_logs'
    ) THEN
        CREATE POLICY tenant_isolation_user_usage_logs ON user_usage_logs
            USING (organization_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;
