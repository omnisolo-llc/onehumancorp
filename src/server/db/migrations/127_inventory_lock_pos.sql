-- +goose Up
-- Migration 127: Add locked_quantity and available_quantity to products

DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        ADD COLUMN IF NOT EXISTS locked_quantity INT DEFAULT 0,
        ADD COLUMN IF NOT EXISTS available_quantity INT DEFAULT 0;

        UPDATE products SET available_quantity = inventory_count WHERE available_quantity = 0 AND inventory_count > 0;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        DROP COLUMN IF NOT EXISTS locked_quantity,
        DROP COLUMN IF NOT EXISTS available_quantity;
    END IF;
END
$$;
