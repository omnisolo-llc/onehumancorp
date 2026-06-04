-- +goose Up
-- Migration 074: Add payment_source to orders

ALTER TABLE orders ADD COLUMN IF NOT EXISTS payment_source TEXT;

-- +goose Down
ALTER TABLE orders DROP COLUMN IF EXISTS payment_source;
