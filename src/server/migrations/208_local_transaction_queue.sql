-- +goose Up
CREATE TABLE IF NOT EXISTS local_transaction_queue (
    transaction_id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending_sync',
    payload JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('local_transaction_queue') IS NOT NULL THEN
        ALTER TABLE local_transaction_queue ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'local_transaction_queue'
                AND policyname = 'tenant_isolation_local_transaction_queue'
        ) THEN
            CREATE POLICY tenant_isolation_local_transaction_queue ON local_transaction_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('local_transaction_queue') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_local_transaction_queue ON local_transaction_queue;
        ALTER TABLE local_transaction_queue DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS local_transaction_queue CASCADE;
