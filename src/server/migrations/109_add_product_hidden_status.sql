-- Migration 109: Add is_hidden to products
-- This allows the Operations Agent to hide products from the online storefront when they sell out.

-- +goose Up
ALTER TABLE products ADD COLUMN IF NOT EXISTS is_hidden BOOLEAN DEFAULT FALSE;

-- +goose Down
ALTER TABLE products DROP COLUMN IF EXISTS is_hidden;
