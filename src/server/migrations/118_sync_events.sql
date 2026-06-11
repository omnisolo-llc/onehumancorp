-- +goose Up
CREATE TABLE IF NOT EXISTS sync_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    mutation_payload JSONB DEFAULT '{}',
    idempotency_key TEXT NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS sync_events_tenant_idempotency_key_idx ON sync_events (tenant_id, idempotency_key);

DO $$
BEGIN
    IF to_regclass('sync_events') IS NOT NULL THEN
        ALTER TABLE sync_events ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'sync_events'
                AND policyname = 'tenant_isolation_sync_events'
        ) THEN
            CREATE POLICY tenant_isolation_sync_events ON sync_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('sync_events') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
        ALTER TABLE sync_events DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS sync_events CASCADE;
