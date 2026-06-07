-- +goose Up
-- Migration 079: Add crdt_deltas and pos_offline_transactions tables for offline synchronization

CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (tenant_id, id)
);

-- Enable RLS for tenant isolation on crdt_deltas
ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;

-- Create policy for tenant isolation on crdt_deltas
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'crdt_deltas'
            AND policyname = 'tenant_isolation_crdt_deltas'
    ) THEN
        CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas
            USING (tenant_id = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Add pos_offline_transactions table
CREATE TABLE IF NOT EXISTS pos_offline_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS for tenant isolation on pos_offline_transactions
ALTER TABLE pos_offline_transactions ENABLE ROW LEVEL SECURITY;

-- Create policy for tenant isolation on pos_offline_transactions
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'pos_offline_transactions'
            AND policyname = 'tenant_isolation_pos_offline_transactions'
    ) THEN
        CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions
            USING (tenant_id = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DROP TABLE IF EXISTS pos_offline_transactions;
DROP TABLE IF EXISTS crdt_deltas;
