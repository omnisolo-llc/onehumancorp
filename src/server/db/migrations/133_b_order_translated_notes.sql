-- +goose Up
-- Migration 133: Add translated_notes and customer_notes to orders, and preferred_language to tenants

DO $$
BEGIN
    IF to_regclass('orders') IS NOT NULL THEN
        ALTER TABLE orders
        ADD COLUMN IF NOT EXISTS customer_notes TEXT,
        ADD COLUMN IF NOT EXISTS translated_notes TEXT;
    END IF;

    IF to_regclass('tenants') IS NOT NULL THEN
        ALTER TABLE tenants
        ADD COLUMN IF NOT EXISTS preferred_language TEXT DEFAULT 'en';
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('orders') IS NOT NULL THEN
        ALTER TABLE orders
        DROP COLUMN IF NOT EXISTS customer_notes,
        DROP COLUMN IF NOT EXISTS translated_notes;
    END IF;

    IF to_regclass('tenants') IS NOT NULL THEN
        ALTER TABLE tenants
        DROP COLUMN IF NOT EXISTS preferred_language;
    END IF;
END
$$;
