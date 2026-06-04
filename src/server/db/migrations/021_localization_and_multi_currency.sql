CREATE TABLE IF NOT EXISTS ohc_fx_rates (
    id TEXT PRIMARY KEY,
    from_currency TEXT NOT NULL,
    to_currency TEXT NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (from_currency, to_currency)
);

CREATE TABLE IF NOT EXISTS ohc_i18n_strings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    locale TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, locale, key)
);

ALTER TABLE ohc_i18n_strings ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_i18n_strings ON ohc_i18n_strings;

CREATE POLICY tenant_isolation_ohc_i18n_strings
ON ohc_i18n_strings
USING (tenant_id = current_setting('app.current_tenant', true) OR tenant_id = 'SYSTEM')
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_multi_currency_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    presentment_amount BIGINT NOT NULL,
    presentment_currency TEXT NOT NULL,
    settlement_amount BIGINT NOT NULL,
    settlement_currency TEXT NOT NULL,
    exchange_rate DOUBLE PRECISION NOT NULL,
    is_offline_sync BOOLEAN NOT NULL DEFAULT false,
    safe_margin_absorbed BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE ohc_multi_currency_ledger ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_multi_currency_ledger ON ohc_multi_currency_ledger;

CREATE POLICY tenant_isolation_ohc_multi_currency_ledger
ON ohc_multi_currency_ledger
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
