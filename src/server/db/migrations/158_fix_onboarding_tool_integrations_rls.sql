-- Fix missing RLS on onboarding_state and tool_integrations tables
-- Ensure they have RLS enabled and a tenant_isolation policy.

DO $$
BEGIN
    -- onboarding_state
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'onboarding_state') THEN
        ALTER TABLE IF EXISTS onboarding_state ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1 FROM pg_policies
            WHERE tablename = 'onboarding_state' AND policyname = 'tenant_isolation_onboarding_state'
        ) THEN
            CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state
                USING (tenant_id = current_setting('app.current_tenant', true)::UUID);
        END IF;
    END IF;

    -- tool_integrations
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'tool_integrations') THEN
        ALTER TABLE IF EXISTS tool_integrations ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1 FROM pg_policies
            WHERE tablename = 'tool_integrations' AND policyname = 'tenant_isolation_tool_integrations'
        ) THEN
            CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations
                USING (tenant_id = current_setting('app.current_tenant', true)::UUID);
        END IF;
    END IF;
END $$;
