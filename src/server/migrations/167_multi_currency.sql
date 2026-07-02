-- +goose Up
-- Migration 167: Add multi-currency fields to invoices, orders, and payment_events

-- 1. Invoices
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD';
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- Ensure existing currency column is migrated or kept in sync (optional depending on use cases)
-- Assuming currency field continues to act as presentment_currency/transaction_currency, we can keep it as is.
-- Here we add base_currency and transaction_currency directly.

-- 2. Orders
ALTER TABLE orders ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE orders ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD';
ALTER TABLE orders ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;

-- 3. Payment Events
ALTER TABLE payment_events ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE payment_events ADD COLUMN IF NOT EXISTS transaction_currency TEXT DEFAULT 'USD';
ALTER TABLE payment_events ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;
