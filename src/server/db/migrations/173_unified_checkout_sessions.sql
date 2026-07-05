-- +goose Up
CREATE TABLE IF NOT EXISTS checkout_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    amount_cents BIGINT NOT NULL,
    device_id TEXT,
    cart_payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO \$\$
BEGIN
    IF to_regclass('checkout_sessions') IS NOT NULL THEN
        ALTER TABLE checkout_sessions ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'checkout_sessions' AND policyname = 'tenant_isolation_checkout_sessions'
        ) THEN
            CREATE POLICY tenant_isolation_checkout_sessions ON checkout_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
\$\$;
