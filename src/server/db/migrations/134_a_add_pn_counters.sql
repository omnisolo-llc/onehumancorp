-- +goose Up
-- Migration 134: Add PN-Counter CRDT logic for product inventory
DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        ADD COLUMN IF NOT EXISTS pn_counter_p INT DEFAULT 0,
        ADD COLUMN IF NOT EXISTS pn_counter_n INT DEFAULT 0;

        -- Initialize positive counter to the current inventory_count.
        -- If inventory_count is null or missing, this ensures we start safe.
        UPDATE products SET pn_counter_p = COALESCE(inventory_count, 0) WHERE pn_counter_p = 0 AND pn_counter_n = 0;

        -- Also, ensure inventory_count matches pn_counter_p - pn_counter_n initially.
        UPDATE products SET inventory_count = GREATEST(0, COALESCE(pn_counter_p, 0) - COALESCE(pn_counter_n, 0));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('products') IS NOT NULL THEN
        ALTER TABLE products
        DROP COLUMN IF NOT EXISTS pn_counter_p,
        DROP COLUMN IF NOT EXISTS pn_counter_n;
    END IF;
END
$$;
