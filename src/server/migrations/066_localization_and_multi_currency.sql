-- Create local_i18n_cache table
CREATE TABLE IF NOT EXISTS local_i18n_cache (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    language_code TEXT NOT NULL,
    translation_key TEXT NOT NULL,
    translation_value TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, language_code, translation_key)
);

CREATE INDEX IF NOT EXISTS idx_local_i18n_cache_tenant_lang ON local_i18n_cache(tenant_id, language_code);

ALTER TABLE local_i18n_cache ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_local_i18n_cache ON local_i18n_cache;
CREATE POLICY tenant_isolation_local_i18n_cache ON local_i18n_cache USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Create offline_fx_rates table
CREATE TABLE IF NOT EXISTS offline_fx_rates (
    id TEXT PRIMARY KEY,
    base_currency TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    exchange_rate NUMERIC(10, 6) NOT NULL,
    fetched_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (base_currency, target_currency)
);

CREATE INDEX IF NOT EXISTS idx_offline_fx_rates_pair ON offline_fx_rates(base_currency, target_currency);
