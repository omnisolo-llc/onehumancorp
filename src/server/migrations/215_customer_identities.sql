CREATE TABLE IF NOT EXISTS customer_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    channel VARCHAR NOT NULL,
    identifier VARCHAR NOT NULL,
    verification_status VARCHAR NOT NULL DEFAULT 'pending',
    trust_score INTEGER DEFAULT 0,
    last_verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Index for fast lookup by identifier and channel
CREATE INDEX IF NOT EXISTS idx_customer_identities_lookup ON customer_identities (tenant_id, channel, identifier);

-- Enforce RLS
ALTER TABLE customer_identities ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'customer_identities'
            AND policyname = 'tenant_isolation_customer_identities'
    ) THEN
        CREATE POLICY tenant_isolation_customer_identities ON customer_identities
        USING (tenant_id = current_setting('app.current_tenant', true))
        WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END
$$;
