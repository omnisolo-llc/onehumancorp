-- +goose Up
-- Add multi-currency capabilities to invoices
ALTER TABLE IF EXISTS invoices ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE IF EXISTS invoices ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD';
ALTER TABLE IF EXISTS invoices ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- Add multi-currency capabilities to cash_ledger_entries
ALTER TABLE IF EXISTS cash_ledger_entries ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE IF EXISTS cash_ledger_entries ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD';
ALTER TABLE IF EXISTS cash_ledger_entries ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- Add multi-currency capabilities to payment_intents
ALTER TABLE IF EXISTS payment_intents ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE IF EXISTS payment_intents ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD';
ALTER TABLE IF EXISTS payment_intents ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- +goose Down
ALTER TABLE IF EXISTS invoices DROP COLUMN IF EXISTS base_currency;
ALTER TABLE IF EXISTS invoices DROP COLUMN IF EXISTS transaction_currency;
ALTER TABLE IF EXISTS invoices DROP COLUMN IF EXISTS exchange_rate;

ALTER TABLE IF EXISTS cash_ledger_entries DROP COLUMN IF EXISTS base_currency;
ALTER TABLE IF EXISTS cash_ledger_entries DROP COLUMN IF EXISTS transaction_currency;
ALTER TABLE IF EXISTS cash_ledger_entries DROP COLUMN IF EXISTS exchange_rate;

ALTER TABLE IF EXISTS payment_intents DROP COLUMN IF EXISTS base_currency;
ALTER TABLE IF EXISTS payment_intents DROP COLUMN IF EXISTS transaction_currency;
ALTER TABLE IF EXISTS payment_intents DROP COLUMN IF EXISTS exchange_rate;
