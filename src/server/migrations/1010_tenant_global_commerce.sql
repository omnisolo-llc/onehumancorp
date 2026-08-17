ALTER TABLE tenants
    ADD COLUMN IF NOT EXISTS base_currency TEXT NOT NULL DEFAULT 'USD',
    ADD COLUMN IF NOT EXISTS enabled_currencies JSONB NOT NULL DEFAULT '["USD"]'::jsonb;

ALTER TABLE tenants
    DROP CONSTRAINT IF EXISTS tenants_base_currency_supported,
    ADD CONSTRAINT tenants_base_currency_supported
        CHECK (base_currency IN ('USD', 'EUR', 'GBP', 'CAD', 'AUD', 'JPY')),
    DROP CONSTRAINT IF EXISTS tenants_enabled_currencies_array,
    ADD CONSTRAINT tenants_enabled_currencies_array
        CHECK (jsonb_typeof(enabled_currencies) = 'array');
