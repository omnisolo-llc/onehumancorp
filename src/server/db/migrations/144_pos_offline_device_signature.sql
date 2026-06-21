-- +goose Up
-- Migration 144: Add device_signature to pos_offline_transactions

DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions
        ADD COLUMN IF NOT EXISTS device_signature TEXT;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions
        DROP COLUMN IF NOT EXISTS device_signature;
    END IF;
END
$$;
