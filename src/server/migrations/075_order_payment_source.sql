-- +goose Up
-- Migration 075: Add payment_source to orders table

ALTER TABLE orders ADD COLUMN IF NOT EXISTS payment_source TEXT;

-- +goose Down
ALTER TABLE orders DROP COLUMN IF EXISTS payment_source;
