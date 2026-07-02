-- +goose Up
-- Add multi-currency support to financial tables
ALTER TABLE orders
ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

ALTER TABLE invoices
ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

ALTER TABLE payment_events
ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

ALTER TABLE tenants
ADD COLUMN IF NOT EXISTS global_sales_enabled BOOLEAN DEFAULT FALSE;

-- +goose Down
ALTER TABLE orders DROP COLUMN IF EXISTS base_currency, DROP COLUMN IF EXISTS transaction_currency, DROP COLUMN IF EXISTS exchange_rate;
ALTER TABLE invoices DROP COLUMN IF EXISTS base_currency, DROP COLUMN IF EXISTS transaction_currency, DROP COLUMN IF EXISTS exchange_rate;
ALTER TABLE payment_events DROP COLUMN IF EXISTS base_currency, DROP COLUMN IF EXISTS transaction_currency, DROP COLUMN IF EXISTS exchange_rate;
ALTER TABLE tenants DROP COLUMN IF EXISTS global_sales_enabled;
