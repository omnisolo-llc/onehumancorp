-- +goose Up
-- Migration 111: Add pos_offline_sync_queue table for offline KDS events

CREATE TABLE IF NOT EXISTS pos_offline_sync_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMPTZ
);

DO $$
BEGIN
    IF to_regclass('pos_offline_sync_queue') IS NOT NULL THEN
        ALTER TABLE pos_offline_sync_queue ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'pos_offline_sync_queue'
                AND policyname = 'tenant_isolation_pos_offline_sync_queue'
        ) THEN
            CREATE POLICY tenant_isolation_pos_offline_sync_queue ON pos_offline_sync_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('pos_offline_sync_queue') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_pos_offline_sync_queue ON pos_offline_sync_queue;
        ALTER TABLE pos_offline_sync_queue DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS pos_offline_sync_queue CASCADE;
