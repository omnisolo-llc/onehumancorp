-- Migration 146: Add client_mutation_id to pos_offline_transactions for idempotency

DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions
        ADD COLUMN IF NOT EXISTS client_mutation_id TEXT;

        CREATE UNIQUE INDEX IF NOT EXISTS idx_pos_offline_transactions_idempotency
        ON pos_offline_transactions (tenant_id, client_mutation_id)
        WHERE client_mutation_id IS NOT NULL;
    END IF;
END
$$;
