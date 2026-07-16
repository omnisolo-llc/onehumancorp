-- Global Offline-First Localization & Currency Engine
-- GitHub Issue #22770
-- Exchange rates for offline-first conversion
CREATE TABLE IF NOT EXISTS ohc_fx_rates (
    id TEXT PRIMARY KEY,
    from_currency TEXT NOT NULL,
    to_currency TEXT NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(from_currency, to_currency)
);
-- Localized UI and AI strings
CREATE TABLE IF NOT EXISTS ohc_i18n_strings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL, -- Supporting per-tenant custom strings
    locale TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, locale, key)
);
CREATE INDEX IF NOT EXISTS idx_ohc_i18n_lookup ON ohc_i18n_strings(tenant_id, locale);
ALTER TABLE ohc_i18n_strings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_i18n_strings ON ohc_i18n_strings;
CREATE POLICY tenant_isolation_ohc_i18n_strings
ON ohc_i18n_strings
USING (tenant_id = 'SYSTEM' OR tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = 'SYSTEM' OR tenant_id = current_setting('app.current_tenant', true));
-- Multi-currency ledger for cross-border reconciliation
CREATE TABLE IF NOT EXISTS ohc_multi_currency_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    presentment_amount BIGINT NOT NULL, -- in cents
    presentment_currency TEXT NOT NULL,
    settlement_amount BIGINT NOT NULL, -- in cents
    settlement_currency TEXT NOT NULL,
    exchange_rate DOUBLE PRECISION NOT NULL,
    is_offline_sync BOOLEAN NOT NULL DEFAULT FALSE,
    safe_margin_absorbed BIGINT NOT NULL DEFAULT 0, -- in cents
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_ohc_multi_currency_ledger_tenant ON ohc_multi_currency_ledger(tenant_id, created_at DESC);
ALTER TABLE ohc_multi_currency_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_multi_currency_ledger ON ohc_multi_currency_ledger;
CREATE POLICY tenant_isolation_ohc_multi_currency_ledger
ON ohc_multi_currency_ledger
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
ALTER TABLE ohc_fx_rates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_fx_rates ON ohc_fx_rates;
CREATE POLICY tenant_isolation_ohc_fx_rates
ON ohc_fx_rates
USING (true)
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);