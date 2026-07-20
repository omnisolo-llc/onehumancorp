CREATE TABLE IF NOT EXISTS onboarding_state (
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    current_step INTEGER NOT NULL DEFAULT 0,
    state_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    PRIMARY KEY (tenant_id, user_id)
);

ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'onboarding_state'
          AND policyname = 'tenant_isolation_onboarding_state'
    ) THEN
        CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;
