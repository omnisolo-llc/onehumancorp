-- +goose Up
CREATE TABLE IF NOT EXISTS ohc_fx_rates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL DEFAULT 'SYSTEM',
    from_currency TEXT NOT NULL,
    to_currency TEXT NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, from_currency, to_currency)
);

DO \$\$
BEGIN
    IF to_regclass('ohc_fx_rates') IS NOT NULL THEN
        ALTER TABLE ohc_fx_rates ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'ohc_fx_rates' AND policyname = 'tenant_isolation_ohc_fx_rates'
        ) THEN
            CREATE POLICY tenant_isolation_ohc_fx_rates ON ohc_fx_rates USING (tenant_id::text = current_setting('app.current_tenant', true) OR tenant_id = 'SYSTEM') WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
\$\$;
