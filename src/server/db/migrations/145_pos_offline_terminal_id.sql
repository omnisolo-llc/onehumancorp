-- +goose Up
-- Migration 145: Add terminal_id to pos_offline_transactions

DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions
        ADD COLUMN IF NOT EXISTS terminal_id TEXT;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions
        DROP COLUMN IF NOT EXISTS terminal_id;
    END IF;
END
$$;
