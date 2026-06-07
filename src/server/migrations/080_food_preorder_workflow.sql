-- +goose Up
-- Migration 080: Food Pre-Order Workflow for OneHumanCorp

ALTER TABLE orders ADD COLUMN IF NOT EXISTS pickup_time TIMESTAMPTZ;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS customer_notes TEXT;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS translated_notes TEXT;

-- +goose Down
-- Reverse Migration 080
ALTER TABLE orders DROP COLUMN IF EXISTS pickup_time;
ALTER TABLE orders DROP COLUMN IF EXISTS customer_notes;
ALTER TABLE orders DROP COLUMN IF EXISTS translated_notes;
