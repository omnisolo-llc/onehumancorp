-- +goose Up
-- Migration 076: Add pos_offline_transactions table

CREATE TABLE IF NOT EXISTS pos_offline_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

DO $$
BEGIN
    IF to_regclass('pos_offline_transactions') IS NOT NULL THEN
        ALTER TABLE pos_offline_transactions ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'pos_offline_transactions'
                AND policyname = 'tenant_isolation_pos_offline_transactions'
        ) THEN
            CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;
