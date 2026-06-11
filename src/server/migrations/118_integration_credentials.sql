-- Migration: Create integration_credentials table
-- Description: Stores credentials for third-party integrations like Twilio with tenant isolation

CREATE TABLE IF NOT EXISTS integration_credentials (
    tenant_id TEXT NOT NULL,
    integration_id TEXT NOT NULL,
    api_token TEXT,
    extra_params JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, integration_id)
);

-- Enable Row Level Security
ALTER TABLE integration_credentials ENABLE ROW LEVEL SECURITY;

-- Create policy for tenant isolation
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE tablename = 'integration_credentials' AND policyname = 'tenant_isolation_policy'
    ) THEN
        CREATE POLICY tenant_isolation_policy ON integration_credentials
            USING (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Ensure settings table has proper RLS as well (proactive fix)
ALTER TABLE settings ENABLE ROW LEVEL SECURITY;

-- Index for faster lookups during webhooks (e.g., looking up tenant by phone number)
CREATE INDEX IF NOT EXISTS idx_integration_credentials_extra_params ON integration_credentials USING gin (extra_params);
