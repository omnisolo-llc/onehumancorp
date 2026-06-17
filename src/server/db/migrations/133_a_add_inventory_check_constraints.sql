-- +goose Up
-- Migration 133: Add check constraints to products table for inventory counts

DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        ADD CONSTRAINT products_inventory_count_check CHECK (inventory_count >= 0),
        ADD CONSTRAINT products_available_quantity_check CHECK (available_quantity >= 0),
        ADD CONSTRAINT products_locked_quantity_check CHECK (locked_quantity >= 0);
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        DROP CONSTRAINT IF EXISTS products_inventory_count_check,
        DROP CONSTRAINT IF EXISTS products_available_quantity_check,
        DROP CONSTRAINT IF EXISTS products_locked_quantity_check;
    END IF;
END
$$;
