-- +goose Up
-- Migration 168: Update pos_offline_transactions for inventory sync

ALTER TABLE pos_offline_transactions
ADD COLUMN IF NOT EXISTS product_id TEXT,
ADD COLUMN IF NOT EXISTS quantity INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS client_timestamp TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS client_transaction_id TEXT;

-- Add a unique constraint to ensure idempotency. If it already exists, this does nothing safely.
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'pos_offline_transactions_client_tx_id_key'
    ) THEN
        ALTER TABLE pos_offline_transactions
        ADD CONSTRAINT pos_offline_transactions_client_tx_id_key UNIQUE (tenant_id, client_transaction_id);
    END IF;
END $$;

-- +goose Down
ALTER TABLE pos_offline_transactions
DROP COLUMN IF EXISTS product_id,
DROP COLUMN IF EXISTS quantity,
DROP COLUMN IF EXISTS client_timestamp,
DROP COLUMN IF EXISTS client_transaction_id;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'pos_offline_transactions_client_tx_id_key'
    ) THEN
        ALTER TABLE pos_offline_transactions
        DROP CONSTRAINT pos_offline_transactions_client_tx_id_key;
    END IF;
END $$;
