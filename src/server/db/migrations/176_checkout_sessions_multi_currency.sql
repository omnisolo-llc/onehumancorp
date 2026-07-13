-- +goose Up
ALTER TABLE checkout_sessions
ADD COLUMN IF NOT EXISTS settlement_amount_cents BIGINT,
ADD COLUMN IF NOT EXISTS settlement_currency TEXT DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- Populate existing rows if any
UPDATE checkout_sessions SET settlement_amount_cents = amount_cents WHERE settlement_amount_cents IS NULL;
