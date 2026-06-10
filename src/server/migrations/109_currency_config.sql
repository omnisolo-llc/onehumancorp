-- CurrencyConfig and Edge localization caching

CREATE TABLE IF NOT EXISTS tenant_currency_configs (
    tenant_id TEXT PRIMARY KEY,
    base_currency TEXT NOT NULL DEFAULT 'USD',
    supported_currencies JSONB NOT NULL DEFAULT '["USD"]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE tenant_currency_configs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_currency_configs ON tenant_currency_configs;
CREATE POLICY tenant_isolation_tenant_currency_configs
ON tenant_currency_configs
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Update Edge triggers or views if needed
