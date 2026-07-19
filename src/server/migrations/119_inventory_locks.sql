-- +goose Up
-- Migration 119: Add locked_quantity and available_quantity to products table

ALTER TABLE products ADD COLUMN IF NOT EXISTS locked_quantity INT DEFAULT 0;
ALTER TABLE products ADD COLUMN IF NOT EXISTS available_quantity INT DEFAULT 0;

-- Set available_quantity to inventory_count for existing products
UPDATE products SET available_quantity = inventory_count WHERE available_quantity = 0;

-- +goose Down
ALTER TABLE products DROP COLUMN IF EXISTS locked_quantity;
ALTER TABLE products DROP COLUMN IF EXISTS available_quantity;
