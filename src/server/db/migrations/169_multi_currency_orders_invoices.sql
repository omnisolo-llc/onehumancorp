-- +goose Up
-- Add multi-currency fields to orders
ALTER TABLE IF EXISTS orders
ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- Add multi-currency fields to invoices
ALTER TABLE IF EXISTS invoices
ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- Add multi-currency fields to payment_events
ALTER TABLE IF EXISTS payment_events
ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- Add global_sales_enabled to tenants
ALTER TABLE IF EXISTS tenants
ADD COLUMN IF NOT EXISTS global_sales_enabled BOOLEAN DEFAULT FALSE;

-- +goose Down
-- Revert added columns
ALTER TABLE IF EXISTS orders
DROP COLUMN IF EXISTS base_currency,
DROP COLUMN IF EXISTS transaction_currency,
DROP COLUMN IF EXISTS exchange_rate;

ALTER TABLE IF EXISTS invoices
DROP COLUMN IF EXISTS base_currency,
DROP COLUMN IF EXISTS transaction_currency,
DROP COLUMN IF EXISTS exchange_rate;

ALTER TABLE IF EXISTS payment_events
DROP COLUMN IF EXISTS base_currency,
DROP COLUMN IF EXISTS transaction_currency,
DROP COLUMN IF EXISTS exchange_rate;

ALTER TABLE IF EXISTS tenants
DROP COLUMN IF EXISTS global_sales_enabled;
