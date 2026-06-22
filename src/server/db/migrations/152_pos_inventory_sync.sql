-- +goose Up
-- Migration 152: Support pos inventory offline sync

DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions
        ADD COLUMN IF NOT EXISTS terminal_session_id TEXT REFERENCES pos_terminal_sessions(id),
        ADD COLUMN IF NOT EXISTS product_id TEXT REFERENCES products(id),
        ADD COLUMN IF NOT EXISTS quantity INT DEFAULT 1;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions
        DROP COLUMN IF NOT EXISTS terminal_session_id,
        DROP COLUMN IF NOT EXISTS product_id,
        DROP COLUMN IF NOT EXISTS quantity;
    END IF;
END
$$;
