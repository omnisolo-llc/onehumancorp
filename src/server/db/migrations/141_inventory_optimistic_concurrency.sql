-- +goose Up
-- Migration 141: Add version to products for optimistic concurrency

DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        ADD COLUMN IF NOT EXISTS version INT DEFAULT 1;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        DROP COLUMN IF NOT EXISTS version;
    END IF;
END
$$;
